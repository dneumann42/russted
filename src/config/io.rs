use std::path::Path;

use crate::config::types::Config;

#[derive(Debug)]
pub enum ConfigIOError {
    Generic(String),
    StdioError(std::io::Error),
    TomlError(toml::de::Error),
}

impl From<std::io::Error> for ConfigIOError {
    fn from(value: std::io::Error) -> Self {
        Self::StdioError(value)
    }
}

impl From<toml::de::Error> for ConfigIOError {
    fn from(value: toml::de::Error) -> Self {
        Self::TomlError(value)
    }
}

pub type Result<T> = std::result::Result<T, ConfigIOError>;

pub trait ConfigWriter {
    fn write<P: AsRef<Path>>(config: &Config, config_path: P) -> Option<ConfigIOError>;
}

pub trait ConfigReader {
    fn read<P: AsRef<Path>>(config_path: P) -> Result<Config>;
}

pub trait ConfigIO: ConfigWriter + ConfigReader {}
