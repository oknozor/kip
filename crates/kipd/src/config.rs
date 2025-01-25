use config::{Config, ConfigError, Environment, File};
use kip_plugin::Plugin;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub plugins: HashMap<String, Plugin>,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config_path = std::env::var("kipD_CONFIG")
            .unwrap_or_else(|_| String::from("~/.config/kipd/config.toml"));

        let expanded_path = shellexpand::tilde(&config_path).to_string();

        Config::builder()
            .add_source(File::with_name(&expanded_path))
            .add_source(Environment::with_prefix("kipD"))
            .build()?
            .try_deserialize()
    }
}
