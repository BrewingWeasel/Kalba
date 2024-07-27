use std::{fs, io, process::Command};

use crate::{SakinyjeError, SakinyjeState};
use shared::ToasterPayload;
use tauri::{State, Window};

#[tauri::command]
pub async fn check_stanza_installed(
    state: State<'_, SakinyjeState>,
) -> Result<bool, SakinyjeError> {
    let state = state.0.lock().await;

    let data_dir = dirs::data_dir()
        .ok_or_else(|| SakinyjeError::MissingDir(String::from("data")))?
        .join("sakinyje")
        .join("stanza");

    Ok(data_dir.exists() && !state.to_save.installing_stanza)
}

#[tauri::command]
pub async fn setup_stanza(
    state: State<'_, SakinyjeState>,
    window: Window,
) -> Result<(), SakinyjeError> {
    let mut state = state.0.lock().await;

    let data_dir = dirs::data_dir()
        .ok_or_else(|| SakinyjeError::MissingDir(String::from("data")))?
        .join("sakinyje")
        .join("stanza");
    // If the directory already exists and we're not resuming an installation, we're done
    if data_dir.exists() && !state.to_save.installing_stanza {
        return Ok(());
    }
    state.to_save.installing_stanza = true;

    fs::create_dir_all(&data_dir)?;

    let python_version_process = Command::new("python").arg("--version").output();
    match python_version_process {
        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
                return Err(SakinyjeError::PythonNotFound);
            }
            return Err(e.into());
        }
        Ok(output) => {
            let stdout_str = String::from_utf8(output.stdout)?;
            let split = stdout_str.split('.').collect::<Vec<_>>();
            if split.len() < 2 {
                return Err(SakinyjeError::PythonNotFound);
            }
            if !split[0].ends_with('3') {
                return Err(SakinyjeError::WrongPythonVersion(stdout_str));
            }
            if let Ok(minor_version) = split[1].parse::<u8>() {
                if minor_version < 8 {
                    return Err(SakinyjeError::WrongPythonVersion(stdout_str));
                }
            } else {
                return Err(SakinyjeError::PythonNotFound);
            }
        }
    }
    window.emit(
        "stanzaDownloadUpdate",
        ToasterPayload {
            message: Some("Creating virtual environment"),
        },
    )?;
    Command::new("python")
        .current_dir(&data_dir)
        .args(["-m", "venv", ".venv"])
        .spawn()?
        .wait()?;

    window.emit(
        "stanzaDownloadUpdate",
        ToasterPayload {
            message: Some("Downloading stanza script"),
        },
    )?;
    let script = reqwest::get(
        "https://raw.githubusercontent.com/brewingweasel/sakinyje/master/stanza/run.py",
    )
    .await?
    .error_for_status()?
    .text()
    .await?;
    fs::write(data_dir.join("run.py"), script)?;

    window.emit(
        "stanzaDownloadUpdate",
        ToasterPayload {
            message: Some("Downloading stanza (this may take a while)"),
        },
    )?;
    Command::new(data_dir.join(".venv").join("bin").join("pip"))
        .current_dir(&data_dir)
        .args(["install", "stanza"])
        .spawn()?
        .wait()?;

    state.to_save.installing_stanza = false;
    Ok(())
}
