use std::path::PathBuf;
use log::{debug, trace};

use crate::{Value, ConfigError, Priority};

/// Trait for read/write configuration files
pub trait ValueFileHandler {
    fn name(&self) -> String;
    fn extensions() -> Vec<String>;
    fn parse(&self, path: &PathBuf) -> Result<Value, ConfigError>;
    fn write(&self, path: &PathBuf, value: &Value) -> Result<(), ConfigError>;
}

/// Struct for file configuration
/// 
/// This struct is used to read/write configuration files.
/// By using an implementation of ValueFileHandler, it is possible to read/write any file format.
/// If the file is not available, the file is created with the initial_value.
/// If the file is required, an error will be returned if the file is not available.
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigFile {
    pub path: PathBuf,
    initial_value: Option<Value>,
    required: bool,
}

impl ConfigFile {

    /// Creates a new ConfigFile
    /// 
    /// The path is the path to the file.
    /// The file is required by default.
    /// The file has no initial value by default.
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            initial_value: None,
            required: true,
        }
    }

    /// Sets the file as required and defines the initial value
    pub fn with_init(mut self, value: Value) -> Self {
        self.set_initial_value(value);
        self.required = true;
        self
    }

    /// Sets the file as required
    pub fn set_required(&mut self, required: bool) {
        self.required = required;
    }

    /// Sets the file as required
    pub fn required(mut self) -> Self {
        self.set_required(true);
        self
    }

    /// Sets the initial value
    pub fn set_initial_value(&mut self, value: Value) {
        let val = match value.is_map() {
            true => Some(value),
            false => None,
        };

        self.initial_value = val;
    }

    pub fn is_required(&self) -> bool {
        self.required
    }

    pub fn get_initial_value(&self) -> Option<&Value> {
        self.initial_value.as_ref()
    }

    /// Checks if the file is available in disk
    pub fn is_available(&self) -> bool {
        match self.path.try_exists() {
            Ok(exists) => exists,
            Err(_) => false,
        }
    }

    pub fn write<T>(&self, handler: &T, value: &Value) -> Result<(), ConfigError> where T: ValueFileHandler {
        debug!("Writing file {:?}", self.path);
        handler.write(&self.path, value)
    }

    pub fn read<T>(&self, handler: &T) -> Result<Value, ConfigError> where T: ValueFileHandler {
        if !self.is_available() {
            debug!("Trying to read file {:?}: is not available", self.path);
            if let Some(value) = &self.initial_value {
                handler.write(&self.path, value)?;
                return Ok(value.clone());
            }

            if self.required {
                return Err(ConfigError::ImportingSource(format!("File {:?} is required", self.path)));
            }

            return Ok(Value::new_map());
        }

        trace!("Reading file {:?}", self.path);
        handler.parse(&self.path)
    }
}

impl From<PathBuf> for ConfigFile {
    fn from(path: PathBuf) -> Self {
        Self::new(path)
    }
}

impl From<&PathBuf> for ConfigFile {
    fn from(path: &PathBuf) -> Self {
        Self::new(path.clone())
    }
}

impl crate::ConfigBuilder {
    /// Adds a file to the configuration by matching the extension with a file handler
    pub fn add_config_file(&mut self, file: ConfigFile, priority: Priority) -> Result<u64, ConfigError> {
        let ext = match file.path.extension() {
            Some(ext) => match ext.to_str() {
                Some(ext) => ext.to_string(),
                None => return Err(ConfigError::FileOperation(format!("File {:?} has an invalid extension", file.path))),
            },
            None => return Err(ConfigError::FileOperation(format!("File {:?} has no extension", file.path))),
        };

        #[cfg(feature = "json")]
        {
            if super::handler_json::JsonConfigHandler::extensions().contains(&ext) {
                return self.add_config(&file.as_json().set_priority(priority));
            }
        }

        #[cfg(feature = "toml")]
        {
            if super::handler_toml::TomlConfigHandler::extensions().contains(&ext) {
                return self.add_config(&file.as_toml().set_priority(priority));
            }
        }

        #[cfg(feature = "yaml")]
        {
            if super::handler_yaml::YamlConfigHandler::extensions().contains(&ext) {
                return self.add_config(&file.as_yaml().set_priority(priority));
            }
        }

        #[cfg(feature = "json5")]
        {
            if super::handler_json5::Json5ConfigHandler::extensions().contains(&ext) {
                return self.add_config(&file.as_json5().set_priority(priority));
            }
        }

        Err(ConfigError::FileOperation(format!("Can't add config file [{0:?}]: handler not found for extension {1:?}", file.path, ext)))
    }
}