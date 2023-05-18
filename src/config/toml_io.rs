use std::fs;
use std::path::Path;

use crate::config::io::{ConfigIO, ConfigReader, ConfigWriter};
use crate::config::types::Config;

use super::io::ConfigIOError;

pub struct TomlIO;

impl ConfigIO for TomlIO {}

impl ConfigWriter for TomlIO {
    fn write<P: AsRef<Path>>(config: &Config, config_path: P) -> Option<ConfigIOError> {
        fs::write(config_path, toml::to_string(config).ok()?)
            .err()
            .map(|it| it.into())
    }
}

impl ConfigReader for TomlIO {
    fn read<P: AsRef<Path>>(config_path: P) -> super::io::Result<Config> {
        let contents = fs::read_to_string(config_path)?;
        Ok(toml::from_str::<Config>(&contents)?)
    }
}
