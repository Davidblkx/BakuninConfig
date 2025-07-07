use crate::config_layer::{ConfigLayerError, Result};

pub struct TomlFileHandler;

impl crate::config_layer::FileHandler for TomlFileHandler {
    fn read(path: &std::path::PathBuf) -> Result<crate::Value> {
        log::trace!("Reading TOML file: {}", path.display());
        let content = std::fs::read_to_string(path)?;
        toml::from_str(&content).map_err(|e| ConfigLayerError::ParseError(e.to_string()))
    }

    fn write(path: &std::path::PathBuf, value: &crate::Value) -> Result<()> {
        log::trace!("Writing TOML file: {}", path.display());
        let content = toml::to_string_pretty(value)
            .map_err(|e| ConfigLayerError::SerializeError(e.to_string()))?;

        std::fs::write(path, content)?;

        Ok(())
    }
}