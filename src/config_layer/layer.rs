use crate::{Result, Value};

pub trait ConfigLayer {
    fn get_name(&self) -> &'static str;

    fn has_value(&self) -> bool;

    fn read_value(&self) -> Result<Value>;

    fn can_write(&self) -> bool;

    fn write_value(&self, value: &Value) -> Result<()>;
}
