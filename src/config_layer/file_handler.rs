use std::path::PathBuf;

use crate::{Result, Value};

/// Trait for file handling operations in configuration layers.
///
/// Implements methods for reading and writing configuration values to files.
pub trait FileHandler {
    fn read(path: &PathBuf) -> Result<Value>;
    fn write(path: &PathBuf, value: &Value) -> Result<()>;
}
