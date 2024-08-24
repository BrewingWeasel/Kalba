use std::process::{Command, Output};

use crate::KalbaError;

pub fn run_command(real_command: &str) -> Result<Output, KalbaError> {
    if cfg!(target_os = "windows") {
        Ok(new_command("cmd").args(["/C", real_command]).output()?)
    } else {
        Ok(Command::new("sh").args(["-c", real_command]).output()?)
    }
}

#[cfg(target_os = "windows")]
pub fn new_command<S>(executable: S) -> Command
where
    S: AsRef<std::ffi::OsStr>,
{
    let mut cmd = Command::new(executable);
    use std::os::windows::process::CommandExt;
    cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW constant
    cmd
}

#[cfg(target_os = "linux")]
pub fn new_command<S>(executable: S) -> Command
where
    S: AsRef<std::ffi::OsStr>,
{
    let mut cmd = Command::new(executable);
    cmd.env("TMPDIR", "/var/tmp");
    cmd
}

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
pub fn new_command<S>(executable: S) -> Command
where
    S: AsRef<std::ffi::OsStr>,
{
    Command::new(executable)
}
