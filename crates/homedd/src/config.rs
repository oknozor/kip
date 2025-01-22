use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub github: GithubSettings,
    pub caldav: CalDavSettings,
    pub daemon: DaemonSettings,
}

#[derive(Debug, Deserialize)]
pub struct GithubSettings {
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct CalDavSettings {
    pub calendar_url: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct DaemonSettings {
    pub socket_path: PathBuf,
    pub fetch_interval: u64,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config_path = std::env::var("HOMEDD_CONFIG")
            .unwrap_or_else(|_| String::from("~/.config/homedd/config.toml"));

        let expanded_path = shellexpand::tilde(&config_path).to_string();

        Config::builder()
            .add_source(File::with_name(&expanded_path))
            .add_source(Environment::with_prefix("HOMEDD"))
            .build()?
            .try_deserialize()
    }
}
