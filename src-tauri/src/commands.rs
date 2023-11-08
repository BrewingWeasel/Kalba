use std::{
    error::Error,
    process::{Command, Output},
};

pub fn run_command(real_command: &str) -> Result<Output, Box<dyn Error>> {
    if cfg!(target_os = "windows") {
        Ok(Command::new("cmd").args(["/C", real_command]).output()?)
    } else {
        Ok(Command::new("sh").args(["-c", real_command]).output()?)
    }
}
