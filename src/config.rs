use std::{fs, path};

use directories::ProjectDirs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub time_format: Option<TimeFormat>,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            time_format: Some(TimeFormat::H24),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, clap::ValueEnum, Deserialize)]
pub enum TimeFormat {
    #[serde(rename = "24h")]
    H24,
    #[serde(rename = "12h")]
    H12,
}

pub fn load_config() -> AppConfig {
    if let Some(proj_dirs) = ProjectDirs::from("", "KhiemNguyen15", "todo") {
        let path = proj_dirs.config_dir().join("config.toml");

        if path.exists() {
            match fs::read_to_string(&path) {
                Ok(content) => {
                    toml::from_str::<AppConfig>(&content).unwrap_or_else(|_| AppConfig::default())
                }

                Err(_) => AppConfig::default(),
            }
        } else {
            AppConfig::default()
        }
    } else {
        AppConfig::default()
    }
}

pub fn get_data_path() -> path::PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from("", "KhiemNguyen15", "todo") {
        let data_dir = proj_dirs.data_local_dir();
        fs::create_dir_all(data_dir).unwrap();

        return data_dir.join("todo.db");
    }

    panic!("Could not determine data directory");
}

pub fn remove_data_dir() {
    if let Some(proj_dirs) = ProjectDirs::from("", "KhiemNguyen15", "todo") {
        let data_dir = proj_dirs.data_local_dir();
        return fs::remove_dir_all(data_dir).unwrap();
    }
}
