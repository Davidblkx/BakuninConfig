use super::ValueIter;
use std::{collections::HashMap, fmt::Display};

use super::ModelError;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    None,
    Boolean(bool),
    Integer(i64),
    LongInteger(i128),
    UInteger(u64),
    ULongInteger(u128),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Map(HashMap<String, Value>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::None => write!(f, "none"),
            Value::Boolean(value) => write!(f, "{}", value),
            Value::Integer(value) => write!(f, "{}", value),
            Value::LongInteger(value) => write!(f, "{}", value),
            Value::UInteger(value) => write!(f, "{}", value),
            Value::ULongInteger(value) => write!(f, "{}", value),
            Value::Float(value) => write!(f, "{}", value),
            Value::String(value) => write!(f, "{}", value),
            Value::Array(value) => {
                write!(f, "[")?;
                for (i, value) in value.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", value)?;
                }
                write!(f, "]")
            }
            Value::Map(value) => {
                write!(f, "{{")?;
                for (i, (key, value)) in value.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", key, value)?;
                }
                write!(f, "}}")
            }
        }
    }
}

impl Value {
    pub fn get(&self, key: &str) -> Value {
        match self {
            Value::Map(map) => match map.get(key) {
                Some(value) => value.to_owned(),
                None => Value::None,
            },
            _ => Value::None,
        }
    }

    pub fn set(&mut self, key: &str, value: Value) -> Result<&mut Self, ModelError> {
        match self {
            Value::Map(map) => {
                map.insert(key.to_string(), value);
                Ok(self)
            }
            _ => Err(ModelError::OperationOnlyForMapValue),
        }
    }

    pub fn at(&self, index: usize) -> Value {
        match self {
            Value::Array(array) => match array.get(index) {
                Some(value) => value.to_owned(),
                None => Value::None,
            },
            _ => Value::None,
        }
    }

    pub fn push(&mut self, value: Value) -> Result<&mut Self, ModelError> {
        match self {
            Value::Array(array) => {
                array.push(value);
                Ok(self)
            }
            _ => Err(ModelError::OperationOnlyForArrayValue),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Value::Array(array) => array.len(),
            Value::Map(map) => map.len(),
            Value::String(string) => string.len(),
            _ => 0,
        }
    }

    pub fn is_map(&self) -> bool {
        matches!(self, Value::Map(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Value::Array(_))
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Value::None)
    }

    pub fn iter<'a>(&'a self) -> ValueIter<'a> {
        ValueIter::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_value_display() {
        let value = Value::None;
        assert_eq!(format!("{}", value), "none");

        let value = Value::Boolean(true);
        assert_eq!(format!("{}", value), "true");

        let value = Value::Integer(1);
        assert_eq!(format!("{}", value), "1");

        let value = Value::LongInteger(1);
        assert_eq!(format!("{}", value), "1");

        let value = Value::UInteger(1);
        assert_eq!(format!("{}", value), "1");

        let value = Value::ULongInteger(1);
        assert_eq!(format!("{}", value), "1");

        let value = Value::Float(1.0);
        assert_eq!(format!("{}", value), "1");

        let value = Value::String("test".to_string());
        assert_eq!(format!("{}", value), "test");

        let value = Value::Array(vec![Value::Integer(1), Value::Integer(2)]);
        assert_eq!(format!("{}", value), "[1, 2]");

        let value = Value::Map(HashMap::from([("a".to_string(), Value::Integer(1))]));
        assert_eq!(format!("{}", value), "{a: 1}");
    }
}
