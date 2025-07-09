use std::collections::HashMap;

use super::ConfigLayerError;
use crate::{Result, Value};

#[derive(Debug, Clone)]
pub struct EnvironmentConfigLayer {
    prefix: &'static str,
}

/// Configuration layer that reads values from environment variables with a specified prefix.
impl EnvironmentConfigLayer {
    pub fn new(prefix: &'static str) -> Self {
        Self {
            prefix: prefix.trim(),
        }
    }

    pub fn get_prefix(&self) -> &'static str {
        self.prefix
    }
}

impl super::ConfigLayer for EnvironmentConfigLayer {
    fn get_name(&self) -> &'static str {
        "Environment"
    }

    fn has_value(&self) -> bool {
        true
    }

    fn can_write(&self) -> bool {
        false
    }

    fn write_value(&self, _value: &Value) -> Result<()> {
        Err(ConfigLayerError::WriteNotSupported.into())
    }

    fn read_value(&self) -> Result<Value> {
        if self.prefix.is_empty() {
            return Err(ConfigLayerError::ErrorReadingValue(
                "Environment prefix cannot be empty".to_string(),
            )
            .into());
        }

        log::trace!(
            "Loading environment variables with prefix {:?}",
            self.prefix
        );
        let mut map = HashMap::new();

        for (key, value) in std::env::vars_os() {
            let str_key = match key.into_string() {
                Ok(str_key) => str_key.trim().to_string(),
                Err(key) => {
                    log::trace!("Invalid environment variable key: {:?}", key);
                    continue;
                }
            };

            if !str_key.starts_with(self.prefix) {
                continue;
            }

            let key = str_key.trim_start_matches(self.prefix);

            let value = match value.into_string() {
                Ok(value) => value,
                Err(value) => {
                    log::debug!(
                        "Invalid environment variable [{:?}] value: {:?}",
                        key,
                        value
                    );
                    continue;
                }
            };

            map.insert(key.to_string(), Value::String(value));
        }

        Ok(Value::Map(map))
    }
}

impl std::fmt::Display for EnvironmentConfigLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EnvironmentConfigLayer[{}]", self.prefix)
    }
}

#[cfg(test)]
mod tests {
    use crate::config_layer::ConfigLayer;

    use super::*;

    #[test]
    fn test_load_environment() {
        std::env::set_var("TEST_value1", "1");
        std::env::set_var("TEST_value2", "2.5");
        std::env::set_var("TEST_value3", "true");
        std::env::set_var("TEST_value4", "data");

        let layer = EnvironmentConfigLayer::new("TEST_");
        let value = layer.read_value().unwrap();

        assert_eq!(value.get("value1").try_into_i64().unwrap(), 1);
        assert_eq!(value.get("value2").try_into_f64().unwrap(), 2.5);
        assert_eq!(value.get("value3").try_into_bool().unwrap(), true);
        assert_eq!(value.get("value4").try_into_string().unwrap(), "data");
    }
}
