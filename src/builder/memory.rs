use crate::{Value, traits::ConfigReader};

use super::{ConfigBuilder, Priority};

/// Config builder for values from memory
pub struct MemoryConfigBuilder {
    pub value: Value,
    pub priority: Priority,
}

impl MemoryConfigBuilder {
    pub fn new() -> Self {
        Self {
            value: Value::new_map(),
            priority: Priority::Any,
        }
    }

    pub fn set_priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }

    pub fn set_value(mut self, value: Value) -> Self {
        self.value = value;
        self
    }
}

impl ConfigReader for MemoryConfigBuilder {
    fn name(&self) -> String {
        format!("Memory[{:#?}]", self.value)
    }

    fn read(&self) -> Result<Value, crate::ConfigError> {
        match self.value.is_map() {
            true => Ok(self.value.clone()),
            false => Err(crate::ConfigError::NotMapValue),
        }
    }

    fn get_priority<T>(&self, handler: &T) -> Option<u64> where T: crate::traits::PriorityHandler {
        self.priority.get_priority(handler)
    }
}

impl ConfigBuilder {
    pub fn append_value(&mut self, value: Value) -> Result<u64, crate::ConfigError> {
        self.add_value(Priority::Any, value)
    }

    pub fn add_value(&mut self, priority: Priority, value: Value) -> Result<u64, crate::ConfigError> {
        let mem = MemoryConfigBuilder {
            value,
            priority,
        };
        self.add_config(&mem)
    }
}

#[cfg(test)]
mod tests {
    use crate::value_map;

    use super::*;

    #[test]
    fn test_memory_config_builder() {
        let builder = MemoryConfigBuilder::new()
            .set_priority(Priority::First)
            .set_value(value_map!{ a: 10 });
        
        assert_eq!(builder.priority, Priority::First);
        assert_eq!(builder.value.get("a").try_into_i64().unwrap(), 10);
    }
}