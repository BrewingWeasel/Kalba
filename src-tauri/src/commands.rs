use std::process::{Command, Output};

use crate::SakinyjeError;

pub fn run_command(real_command: &str) -> Result<Output, SakinyjeError> {
    if cfg!(target_os = "windows") {
        Ok(Command::new("cmd").args(["/C", real_command]).output()?)
    } else {
        Ok(Command::new("sh").args(["-c", real_command]).output()?)
    }
}
