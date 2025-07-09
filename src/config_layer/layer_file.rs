use std::path::PathBuf;

use super::FileHandler;
use crate::{Result, Value};

#[derive(Debug, Clone)]
/// A configuration layer that reads/write from a file.
pub struct FileConfigLayer<T>
where
    T: FileHandler,
{
    path: PathBuf,
    t: std::marker::PhantomData<T>,
}

impl<T> FileConfigLayer<T>
where
    T: FileHandler,
{
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            t: std::marker::PhantomData,
        }
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn exists(&self) -> bool {
        self.path.exists()
    }
}

impl<T> super::ConfigLayer for FileConfigLayer<T>
where
    T: FileHandler,
{
    fn get_name(&self) -> &'static str {
        "File"
    }

    fn has_value(&self) -> bool {
        self.path.exists()
    }

    fn can_write(&self) -> bool {
        false
    }

    fn write_value(&self, value: &Value) -> Result<()> {
        T::write(&self.path, value)
    }

    fn read_value(&self) -> Result<Value> {
        if !self.path.exists() {
            return Ok(Value::new_map());
        }

        T::read(self.get_path())
    }
}
