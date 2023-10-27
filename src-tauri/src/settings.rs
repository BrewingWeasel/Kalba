use shared::*;
use std::{error::Error, fs};

#[tauri::command]
pub async fn get_def(settings: Dictionary) {}

