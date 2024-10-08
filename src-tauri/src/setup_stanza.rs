use std::{fs, io};

use crate::{commands::new_command, KalbaError, KalbaState};
use shared::ToasterPayload;
use tauri::{Emitter, State, Window};

#[tauri::command]
pub async fn check_stanza_installed(state: State<'_, KalbaState>) -> Result<bool, KalbaError> {
    let state = state.0.lock().await;

    let data_dir = dirs::data_dir()
        .ok_or_else(|| KalbaError::MissingDir(String::from("data")))?
        .join("kalba")
        .join("stanza");

    Ok(data_dir.exists() && !state.to_save.installing_stanza)
}

#[tauri::command]
pub async fn setup_stanza(state: State<'_, KalbaState>, window: Window) -> Result<(), KalbaError> {
    let mut state = state.0.lock().await;

    let python_command = {
        let mut current_command = Err(KalbaError::PythonNotFound);
        for command in &["python", "python3"] {
            let output = new_command(command).spawn();
            if let Err(e) = output {
                if e.kind() == io::ErrorKind::NotFound {
                    continue;
                }
                current_command = Err(e.into());
            } else {
                current_command = Ok(command);
                break;
            }
        }
        current_command
    }?;

    let data_dir = dirs::data_dir()
        .ok_or_else(|| KalbaError::MissingDir(String::from("data")))?
        .join("kalba")
        .join("stanza");
    // If the directory already exists and we're not resuming an installation, we're done
    if data_dir.exists() && !state.to_save.installing_stanza {
        return Ok(());
    }
    state.to_save.installing_stanza = true;

    fs::create_dir_all(&data_dir)?;

    let python_version_process = new_command(python_command).arg("--version").output();
    match python_version_process {
        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
                return Err(KalbaError::PythonNotFound);
            }
            return Err(e.into());
        }
        Ok(output) => {
            let stdout_str = String::from_utf8(output.stdout)?;
            let split = stdout_str.split('.').collect::<Vec<_>>();
            if split.len() < 2 {
                return Err(KalbaError::PythonNotFound);
            }
            if !split[0].ends_with('3') {
                return Err(KalbaError::WrongPythonVersion(stdout_str));
            }
            if let Ok(minor_version) = split[1].parse::<u8>() {
                if minor_version < 8 {
                    return Err(KalbaError::WrongPythonVersion(stdout_str));
                }
            } else {
                return Err(KalbaError::PythonNotFound);
            }
        }
    }
    window.emit(
        "stanzaDownloadUpdate",
        ToasterPayload {
            message: Some("Creating virtual environment"),
        },
    )?;
    new_command("python")
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
    let script =
        reqwest::get("https://raw.githubusercontent.com/brewingweasel/kalba/master/stanza/run.py")
            .await?
            .error_for_status()?
            .text()
            .await?;
    fs::write(data_dir.join("run.py"), script)?;

    let requirements = reqwest::get(
        "https://raw.githubusercontent.com/brewingweasel/kalba/master/stanza/requirements.txt",
    )
    .await?
    .error_for_status()?
    .text()
    .await?;
    fs::write(data_dir.join("requirements.txt"), requirements)?;

    window.emit(
        "stanzaDownloadUpdate",
        ToasterPayload {
            message: Some("Downloading stanza (this may take a while)"),
        },
    )?;
    let exit_status = new_command(
        data_dir
            .join(".venv")
            .join(if cfg!(target_os = "windows") {
                "Scripts"
            } else {
                "bin"
            })
            .join("pip"),
    )
    .current_dir(&data_dir)
    .args(["install", "-r", "requirements.txt"])
    .spawn()?
    .wait()?;

    if !exit_status.success() {
        return Err(KalbaError::PipInstallFailed);
    }

    state.to_save.installing_stanza = false;
    Ok(())
}

#[tauri::command]
pub async fn uninstall_stanza() -> Result<(), KalbaError> {
    let data_dir = dirs::data_dir()
        .ok_or_else(|| KalbaError::MissingDir(String::from("data")))?
        .join("kalba")
        .join("stanza");

    std::fs::remove_dir_all(data_dir)?;
    Ok(())
}
