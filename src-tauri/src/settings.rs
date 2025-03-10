// CS2 Tapping Helper — Settings module
// Author: Dmitry Osin <d@osin.pro>

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub key_left: String,
    pub key_right: String,
    pub key_forward: String,
    pub key_back: String,
    pub delay_min_ms: u64,
    pub delay_max_ms: u64,
    pub strafe_forward_back: bool,
    pub start_active: bool,
    pub start_minimized: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            key_left: "KeyA".into(),
            key_right: "KeyD".into(),
            key_forward: "KeyW".into(),
            key_back: "KeyS".into(),
            delay_min_ms: 70,
            delay_max_ms: 89,
            strafe_forward_back: false,
            start_active: false,
            start_minimized: false,
        }
    }
}

pub fn settings_path() -> PathBuf {
    let mut path = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."));
    path.push("cs2tapping");
    path.push("settings.json");
    path
}

pub fn load_settings() -> Settings {
    let path = settings_path();
    if let Ok(data) = fs::read_to_string(&path) {
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        Settings::default()
    }
}

pub fn save_settings(settings: &Settings) -> Result<(), String> {
    let path = settings_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())
}
