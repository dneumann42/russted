use crate::config::types::Config;

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
    fn write(config: &Config, config_path: &std::path::Path) -> Option<ConfigIOError>;
}

pub trait ConfigReader {
    fn read(config_path: &std::path::Path) -> Result<Config>;
}

pub trait ConfigIO: ConfigWriter + ConfigReader {}
