use std::path::PathBuf;

use crate::{Value, ConfigError, config_file_error, deserialize_error, serialize_error, impl_file_handler, impl_file_extensions, impl_file_name};

use super::file::ValueFileHandler;

pub struct TomlConfigHandler {}

impl ValueFileHandler for TomlConfigHandler {
    impl_file_name!(toml);

    impl_file_extensions!(toml);

    fn parse(&self, path: &PathBuf) -> Result<Value, ConfigError> {
        let content = std::fs::read_to_string(path).map_err(|e| config_file_error!(e.to_string()))?;
        toml::from_str(&content).map_err(|e| deserialize_error!(e.to_string()))
    }

    fn write(&self, path: &PathBuf, value: &Value) -> Result<(), ConfigError> {
        let content = toml::to_string_pretty(value).map_err(|e| serialize_error!(e.to_string()))?;
        std::fs::write(path, content).map_err(|e| config_file_error!(e.to_string()))
    }
}

impl_file_handler!(TomlConfigHandler, toml);