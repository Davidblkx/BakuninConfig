use crate::{Value, Priority};
use crate::traits::ConfigReader;

use super::file::{ConfigFile, ValueFileHandler};

/// Config builder for values from files
pub struct FileConfigBuilder<T> where T: ValueFileHandler + Sized {
    pub file: ConfigFile,
    pub priority: Priority,
    handler: T,
}

impl<T> FileConfigBuilder<T> where T: ValueFileHandler + Sized {
    pub fn new(file: ConfigFile, handler: T) -> Self {
        Self {
            file,
            handler,
            priority: Priority::Any,
        }
    }

    pub fn set_priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }

    pub fn write(&self, value: &Value) -> Result<(), crate::ConfigError> {
        self.file.write(&self.handler, value)
    }
}

impl<T: ValueFileHandler> ConfigReader for FileConfigBuilder<T> {
    fn name(&self) -> String {
        format!("{0:?}[{1:?}]", self.handler.name(), self.file.path)
    }

    fn read(&self) -> Result<Value, crate::ConfigError> {
        self.file.read(&self.handler)
    }

    fn get_priority<H>(&self, handler: &H) -> Option<u64> where H: crate::traits::PriorityHandler {
        self.priority.get_priority(handler)
    }
}