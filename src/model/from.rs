use std::collections::HashMap;

use super::Value;

impl Default for Value {
    fn default() -> Self {
        Value::None
    }
}

impl<T> From<Option<T>> for Value
where
    T: Into<Self>,
{
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => value.into(),
            None => Self::None,
        }
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Self {
        Self::Integer(value.into())
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Self::Integer(value.into())
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self::Integer(value.into())
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<i128> for Value {
    fn from(value: i128) -> Self {
        Self::LongInteger(value)
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Self::UInteger(value.into())
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Self::UInteger(value.into())
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Self::UInteger(value.into())
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Self::UInteger(value)
    }
}

impl From<u128> for Value {
    fn from(value: u128) -> Self {
        Self::ULongInteger(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Self::Float(value as f64)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl<T> From<HashMap<String, T>> for Value
where
    T: Into<Value>,
{
    fn from(values: HashMap<String, T>) -> Self {
        Self::Map(values.into_iter().map(|(k, v)| (k, v.into())).collect())
    }
}

impl<T> From<Vec<T>> for Value
where
    T: Into<Value>,
{
    fn from(values: Vec<T>) -> Self {
        Self::Array(values.into_iter().map(T::into).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_value_from() {
        let value = Value::from(1);
        assert_eq!(value, Value::Integer(1));

        let value = Value::from(1.0);
        assert_eq!(value, Value::Float(1.0));

        let value = Value::from(true);
        assert_eq!(value, Value::Boolean(true));

        let value = Value::from("test");
        assert_eq!(value, Value::String("test".to_string()));

        let value = Value::from(vec![1, 2, 3]);
        assert_eq!(
            value,
            Value::Array(vec![Value::Integer(1), Value::Integer(2), Value::Integer(3)])
        );

        let mut map = HashMap::new();
        map.insert("test".to_string(), 1);
        let value = Value::from(map);
        assert_eq!(
            value,
            Value::Map(vec![("test".to_string(), Value::Integer(1))].into_iter().collect())
        );
    }
}