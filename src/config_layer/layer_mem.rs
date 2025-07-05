use crate::Value;
use super::{ConfigLayerError, Result};

#[derive(Debug, Clone)]
pub struct MemoryConfigLayer {
    value: Value,
}

/// Static value configuration layer.
impl MemoryConfigLayer {
    pub fn new(value: Value) -> Self {
        Self { value }
    }
}

impl super::ConfigLayer for MemoryConfigLayer {
    fn get_name(&self) -> &'static str {
        "Memory"
    }

    fn has_value(&self) -> bool {
        true
    }

    fn can_write(&self) -> bool {
        false
    }

    fn write_value(&self, _value: &Value) -> Result<()> {
        Err(ConfigLayerError::WriteNotSupported)
    }

    fn read_value(&self) -> Result<Value> {
        Ok(self.value.clone())
    }
}
