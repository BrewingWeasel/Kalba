use chrono::{Datelike, TimeDelta, Utc};
use shared::{NumWordsKnown, TimeSpentPoint, TimeSpentStats};
use tauri::State;

use crate::KalbaState;

#[tauri::command]
pub async fn time_spent(state: State<'_, KalbaState>) -> Result<TimeSpentStats, String> {
    let state = state.0.lock().await;

    let mut time_spent_days_this_week = [TimeDelta::default(); 7];
    let mut time_spent_this_week = TimeDelta::default();
    let mut time_spent_this_month = TimeDelta::default();
    let mut time_spent_this_year = TimeDelta::default();
    let mut total_time_spent = TimeDelta::default();
    let mut streak = 0;

    for (time, _) in state.to_save.sessions.iter().rev() {
        let days_since = Utc::now().signed_duration_since(*time).num_days();
        if days_since == streak {
            streak += 1;
        }
    }

    for (time, duration) in state.to_save.sessions.iter() {
        let days_since = Utc::now().signed_duration_since(*time).num_days();
        let duration = TimeDelta::from_std(*duration).expect("duration to be valid");

        for day in 0..7 {
            if days_since == day {
                time_spent_days_this_week[day as usize] += duration;
            }
        }

        if days_since < 7 {
            time_spent_this_week += duration;
        }
        if days_since < 30 {
            time_spent_this_month += duration;
        }
        if days_since < 365 {
            time_spent_this_year += duration;
        }
        total_time_spent += duration;
    }

    let mut current_weekday = chrono::offset::Local::now().weekday().succ();
    let mut days_this_week = Vec::new();
    for time_spent in time_spent_days_this_week.iter().rev() {
        days_this_week.push(TimeSpentPoint {
            name: current_weekday.to_string(),
            duration: time_spent.num_minutes(),
        });
        current_weekday = current_weekday.succ();
    }

    Ok(TimeSpentStats {
        days_this_week,
        total_this_week: formatted_duration(&time_spent_this_week),
        this_month: formatted_duration(&time_spent_this_month),
        this_year: formatted_duration(&time_spent_this_year),
        total: formatted_duration(&total_time_spent),
        streak,
    })
}

const WORD_RATING: [&str; 4] = ["Learning", "Recognized", "Familiar", "Known"];

#[tauri::command]
pub async fn get_words_known_at_levels(
    state: State<'_, KalbaState>,
) -> Result<Vec<NumWordsKnown>, String> {
    let state = state.0.lock().await;
    let current_language = state.current_language.as_ref().expect("language to be set");
    log::info!("loading stats for profile {current_language}");
    let words = &state
        .to_save
        .language_specific
        .get(current_language)
        .expect("language to include")
        .words;
    let mut words_at_rating = [0; 4];
    for info in words.values() {
        match info.rating {
            -1 | 0 => (),
            v => {
                words_at_rating[v as usize - 1] += 1;
            }
        }
    }
    Ok(words_at_rating
        .iter()
        .enumerate()
        .map(|(rating, amount)| NumWordsKnown {
            name: format!("{} words", WORD_RATING[rating]),
            amount: *amount,
        })
        .collect())
}

#[tauri::command]
pub async fn get_words_added(state: State<'_, KalbaState>) -> Result<[usize; 4], String> {
    let state = state.0.lock().await;
    let current_language = state.current_language.as_ref().expect("language to be set");
    log::info!("loading stats for profile {current_language}");
    let words_added = &state
        .to_save
        .language_specific
        .get(current_language)
        .expect("language to include")
        .added_to_anki;
    let mut words_added_at_times = [0; 4];
    for (time, _) in words_added.iter().rev() {
        let days_since = Utc::now().signed_duration_since(*time).num_days();
        if days_since < 7 {
            words_added_at_times[0] += 1;
        }
        if days_since < 30 {
            words_added_at_times[1] += 1;
        }
        if days_since < 365 {
            words_added_at_times[2] += 1;
        }
        words_added_at_times[3] += 1;
    }
    Ok(words_added_at_times)
}

fn formatted_duration(duration: &TimeDelta) -> (String, String) {
    let minutes = duration.num_minutes() as f64;
    if minutes < 60.0 {
        (
            format!(
                "{:.2}",
                minutes + ((duration.num_seconds() % 60) as f64 / 60.0)
            ),
            String::from('m'),
        )
    } else {
        let hours = format!(
            "{:.2}",
            duration.num_hours() as f64 + ((minutes % 60.0) / 60.0)
        );
        (hours, String::from('h'))
    }
}
