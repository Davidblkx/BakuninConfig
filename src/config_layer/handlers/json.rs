use crate::config_layer::{ConfigLayerError, Result};

pub struct JsonFileHandler;

impl crate::config_layer::FileHandler for JsonFileHandler {
    fn read(path: &std::path::PathBuf) -> Result<crate::Value> {
        log::trace!("Reading JSON file: {}", path.display());
        let content = std::fs::read_to_string(path)?;
        json5::from_str(&content).map_err(|e| ConfigLayerError::ParseError(e.to_string()))
    }

    fn write(path: &std::path::PathBuf, value: &crate::Value) -> Result<()> {
        log::trace!("Writing JSON file: {}", path.display());
        let content = json5::to_string(value)
            .map_err(|e| ConfigLayerError::SerializeError(e.to_string()))?;

        std::fs::write(path, content)?;

        Ok(())
    }
}