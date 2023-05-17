use std::fs;

use crate::config::io::{ConfigIO, ConfigReader, ConfigWriter};
use crate::config::types::Config;

use super::io::ConfigIOError;

struct TomlIO;

impl ConfigIO for TomlIO {}

impl ConfigWriter for TomlIO {
    fn write(config: &Config, config_path: &std::path::Path) -> Option<ConfigIOError> {
        fs::write(config_path, toml::to_string(config).ok()?)
            .err()
            .map(|it| it.into())
    }
}

impl ConfigReader for TomlIO {
    fn read(config_path: &std::path::Path) -> super::io::Result<Config> {
        let contents = fs::read_to_string(config_path)?;
        Ok(toml::from_str::<Config>(&contents)?)
    }
}
