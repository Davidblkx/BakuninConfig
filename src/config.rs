use std::collections::HashMap;

use crate::config_layer::{
    handlers, ConfigLayer, EnvironmentConfigLayer, FileConfigLayer, MemoryConfigLayer,
};
use crate::file_finder::FileExtension;
use crate::{Result, Value};

/// A configuration builder that allows adding multiple configuration layers
/// and building a final configuration value by merging the values from all layers.
pub struct BakuninConfig {
    layers_names: Vec<&'static str>,
    layers: HashMap<&'static str, Box<dyn ConfigLayer>>,
}

impl BakuninConfig {
    pub fn new() -> Self {
        BakuninConfig {
            layers_names: Vec::new(),
            layers: HashMap::new(),
        }
    }

    /// Add a new configuration layer to the builder.
    ///
    /// If a layer with the same name already exists, it will be ignored.
    ///
    /// Layers are identified by their names, which are static string slices.
    pub fn push_layer(&mut self, name: &'static str, layer: Box<dyn ConfigLayer>) {
        if self.layers.contains_key(name) {
            log::trace!(
                "Layer '{}' already exists in config, skipping addition",
                name
            );
            return; // Skip adding if the layer already exists
        }

        self.layers_names.push(name);
        self.layers.insert(name, layer);
    }

    /// Add a new configuration layer.
    ///
    /// Like `push_layer`, but returns `self` for method chaining.
    pub fn with_layer(mut self, name: &'static str, layer: Box<dyn ConfigLayer>) -> Self {
        self.push_layer(name, layer);
        self
    }

    /// Adds a file to the configuration layers based on its file extension.
    ///
    /// Because the file extension determines which handler to use, this function
    /// will return an error if the file extension is not supported.
    pub fn add_file_layer(&mut self, name: &'static str, path: std::path::PathBuf) -> Result<()> {
        #[cfg(feature = "toml")]
        {
            if FileExtension::Toml.match_path(&path) {
                self.push_layer(
                    name,
                    Box::new(FileConfigLayer::<handlers::TomlFileHandler>::new(path)),
                );
                return Ok(());
            }
        }

        #[cfg(feature = "json")]
        {
            if FileExtension::Json.match_path(&path) {
                self.push_layer(
                    name,
                    Box::new(FileConfigLayer::<handlers::JsonFileHandler>::new(path)),
                );
                return Ok(());
            }
        }

        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        return Err(crate::config_layer::ConfigLayerError::UnsupportedExtension(ext.into()).into());
    }

    /// Adds a file layer to the configuration builder.
    ///
    /// This method is a convenience wrapper around `add_file_layer` that returns `Result<Self>`.
    pub fn with_file_layer(mut self, name: &'static str, path: std::path::PathBuf) -> Result<Self> {
        self.add_file_layer(name, path)?;
        Ok(self)
    }

    /// Adds an environment variable layer to the configuration builder.
    ///
    /// Parameters:
    /// - `name`: The name of the layer, used for identification.
    /// - `prefix`: The prefix for environment variables that this layer will read.
    pub fn add_environment_layer(&mut self, name: &'static str, prefix: &'static str) {
        self.push_layer(name, Box::new(EnvironmentConfigLayer::new(prefix)));
    }

    /// Adds an environment variable layer to the configuration builder.
    ///
    /// This method is a convenience wrapper around `add_environment_layer` that returns `Self`.
    pub fn with_environment_layer(mut self, name: &'static str, prefix: &'static str) -> Self {
        self.add_environment_layer(name, prefix);
        self
    }

    /// Adds a memory layer to the configuration builder.
    ///
    /// This layer is useful for testing or when you want to provide a default configuration
    pub fn add_memory_layer(&mut self, name: &'static str, value: Value) {
        self.push_layer(name, Box::new(MemoryConfigLayer::new(value)));
    }

    /// Adds a memory layer to the configuration builder.
    ///
    /// This method is a convenience wrapper around `add_memory_layer` that returns `Self`.
    pub fn with_memory_layer(mut self, name: &'static str, value: Value) -> Self {
        self.add_memory_layer(name, value);
        self
    }

    pub fn get_layer(&self, name: &'static str) -> Option<&Box<dyn ConfigLayer>> {
        self.layers.get(name)
    }

    /// Builds the configuration value by reading from all layers in the order they were added.
    /// If `skip_on_error` is true, it will skip layers that return an error
    pub fn build_value(&self, skip_on_error: bool) -> Result<Value> {
        let mut value = Value::new_map();

        for name in &self.layers_names {
            if let Some(layer) = self.layers.get(name) {
                let layer_value = layer.read_value();
                if let Err(e) = layer_value {
                    if skip_on_error {
                        log::warn!("Error reading config layer '{}': {}", name, e);
                        continue; // Skip this layer on error
                    } else {
                        return Err(e);
                    }
                } else if let Ok(layer_value) = layer_value {
                    // Merge the layer value into the main value
                    value.merge(&layer_value);
                }
            }
        }

        Ok(value)
    }
}

impl std::fmt::Debug for BakuninConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut layers = Vec::new();

        for (name, layer) in &self.layers {
            layers.push((name, layer.get_name()));
        }

        f.debug_struct("BakuninConfig")
            .field("layers", &layers)
            .finish()
    }
}
