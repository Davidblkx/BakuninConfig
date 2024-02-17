use std::path::PathBuf;

use crate::{Value, ConfigError, config_file_error, deserialize_error, serialize_error, impl_file_handler, impl_file_extensions, impl_file_name};

use super::file::ValueFileHandler;

pub struct JsonConfigHandler {}

impl ValueFileHandler for JsonConfigHandler {
    impl_file_name!(json);
    
    impl_file_extensions!(json);

    fn parse(&self, path: &PathBuf) -> Result<Value, ConfigError> {
        let reader = std::fs::File::open(path).map_err(|e| config_file_error!(e.to_string()))?;
        serde_json::from_reader(reader).map_err(|e| deserialize_error!(e.to_string()))
    }

    fn write(&self, path: &PathBuf, value: &Value) -> Result<(), ConfigError> {
        let writer = std::fs::File::create(path).map_err(|e| config_file_error!(e.to_string()))?;
        serde_json::to_writer_pretty(writer, value).map_err(|e| serialize_error!(e.to_string()))
    }
}

impl_file_handler!(JsonConfigHandler, json);
