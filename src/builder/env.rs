use std::{env, collections::HashMap};
use log::{warn, trace};

use crate::{Value, ConfigError, traits::ConfigReader};

use super::Priority;

/// Config builder for values from environment
pub struct EnvironmentConfigBuilder {
    pub prefix: String,
    pub priority: Priority,
    value: Value,
}

impl EnvironmentConfigBuilder {
    pub fn new(prefix: String) -> Self {
        Self {
            prefix: Self::build_prefix(&prefix),
            priority: Priority::Any,
            value: Value::None,
        }
    }

    fn build_prefix(prefix: &str) -> String {
        let mut prefix = prefix.trim().to_string();
        if !prefix.is_empty() && !prefix.ends_with("_") {
            prefix.push('_');
        }
        prefix
    }

    pub fn set_prefix(mut self, prefix: &str) -> Self {
        self.prefix = Self::build_prefix(prefix);
        self
    }

    pub fn set_priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }

    /** Reads the configuration from environment and saves to cache */
    pub fn load_mut(&mut self) -> Result<(), ConfigError> {
        self.value = self.read()?;
        Ok(())
    }

    pub fn cache(&self) -> &Value {
        &self.value
    }
}

impl ConfigReader for EnvironmentConfigBuilder {
    fn name(&self) -> String {
        format!("Environment[{}]", self.prefix)
    }

    fn read(&self) -> Result<Value, ConfigError> {
        if self.value.is_map() {
            return Ok(self.value.clone())
        }

        let prefix = self.prefix.trim();

        if prefix.is_empty() {
            return Err(ConfigError::Environment("Environment prefix cannot be empty".to_string()));
        }
    
        trace!("Loading environment variables with prefix {:?}", prefix);
        let mut map = HashMap::new();
    
        for (key, value) in env::vars_os() {
            let str_key = match key.into_string() {
                Ok(str_key) => str_key.trim().to_string(),
                Err(key) => {
                    warn!("Invalid environment variable key: {:?}", key);
                    continue;
                }
            };
    
            if !str_key.starts_with(prefix) {
                continue;
            }
    
            let key = str_key.trim_start_matches(prefix);
    
            let value = match value.into_string() {
                Ok(value) => value,
                Err(value) => {
                    warn!("Invalid environment variable value: {:?}", value);
                    continue;
                }
            };
    
            map.insert(key.to_string(), Value::String(value));
        }
    
        Ok(Value::Map(map))
    }

    fn get_priority<T>(&self, handler: &T) -> Option<u64> where T: crate::traits::PriorityHandler {
        self.priority.get_priority(handler)
    }
}

impl super::ConfigBuilder {
    /** Adds a new environment configuration */
    pub fn add_environment(&mut self, priority: Priority, prefix: &str) -> Result<u64, ConfigError> {
        let env = EnvironmentConfigBuilder::new(prefix.to_string())
            .set_priority(priority);
        self.add_config(&env)
    }

    /** Adds a new environment configuration to the end of priority (last to merge) */
    pub fn environment(&mut self, prefix: &str) -> Result<u64, ConfigError> {
        self.add_environment(Priority::LastAvailable, prefix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_environment() {
        env::set_var("TEST_value1", "1");
        env::set_var("TEST_value2", "2.5");
        env::set_var("TEST_value3", "true");
        env::set_var("TEST_value4", "data");

        let mut loader = EnvironmentConfigBuilder::new("TEST_".to_string());
        loader.load_mut().unwrap();
        let value = loader.cache();

        assert_eq!(value.get("value1").try_into_i64().unwrap(), 1);
        assert_eq!(value.get("value2").try_into_f64().unwrap(), 2.5);
        assert_eq!(value.get("value3").try_into_bool().unwrap(), true);
        assert_eq!(value.get("value4").try_into_string().unwrap(), "data");
    }
}