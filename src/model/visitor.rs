use std::{fmt, collections::HashMap};

use serde::de::{self, Visitor};

use super::Value;

struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid configuration value")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
    where
        E: de::Error {
        Ok(Value::Boolean(value))
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
        where
            E: de::Error, {
        Ok(Value::Integer(v as i64))
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
        where
            E: de::Error, {
        Ok(Value::Integer(v as i64))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
        where
            E: de::Error, {
        Ok(Value::Integer(v as i64))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error {
        Ok(Value::Integer(value))
    }

    fn visit_i128<E>(self, value: i128) -> Result<Self::Value, E>
    where
        E: de::Error {
        Ok(Value::LongInteger(value))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
        where
            E: de::Error, {
        Ok(Value::UInteger(v as u64))
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
        where
            E: de::Error, {
        Ok(Value::UInteger(v as u64))
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
        where
            E: de::Error, {
        Ok(Value::UInteger(v as u64))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error {
        Ok(Value::UInteger(value))
    }

    fn visit_u128<E>(self, value: u128) -> Result<Self::Value, E>
    where
        E: de::Error {
        Ok(Value::ULongInteger(value))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error {
        Ok(Value::Float(value))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error {
        Ok(Value::String(value.to_owned()))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error {
        Ok(Value::String(value))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error {
        Ok(Value::None)
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
        where
            E: de::Error, {
        Ok(Value::String(v.to_owned()))
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
        where
            E: de::Error, {
        Ok(Value::Float(v as f64))
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error, {
        Ok(Value::None)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: serde::Deserializer<'de>, {
        deserializer.deserialize_any(self)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>, {
        let mut vec = Vec::<Value>::new();

        while let Some(elem) = seq.next_element()? {
            vec.push(elem);
        }

        Ok(Value::Array(vec))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: de::MapAccess<'de>, {
        let mut hash_map = HashMap::<String, Value>::new();

        while let Some((key, value)) = map.next_entry()? {
            hash_map.insert(key, value);
        }

        Ok(Value::Map(hash_map))
    }
}

impl<'de> serde::de::Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>
        where
            D: serde::Deserializer<'de>, {
        deserializer.deserialize_any(ValueVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_value_deserialize_none() {
        let value = serde_json::from_str::<Value>("null").unwrap();
        assert_eq!(value, Value::None);
    }

    #[test]
    fn test_model_value_deserialize_boolean() {
        let value = serde_json::from_str::<Value>("true").unwrap();
        assert_eq!(value, Value::Boolean(true));
    }

    #[test]
    fn test_model_value_deserialize_integer() {
        let value: i64 = serde_json::from_str::<Value>("42").unwrap().try_into().unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn test_model_value_deserialize_long_integer() {
        let value: u128 = serde_json::from_str::<Value>("9223372036854775808").unwrap().try_into().unwrap();
        assert_eq!(value, 9223372036854775808);
    }

    #[test]
    fn test_model_value_deserialize_unsigned_integer() {
        let value: u64 = serde_json::from_str::<Value>("556").unwrap().try_into().unwrap();
        assert_eq!(value, 556);
    }

    #[test]
    fn test_model_value_deserialize_unsigned_long_integer() {
        let value: u128 = serde_json::from_str::<Value>("18446744073709551616").unwrap().try_into().unwrap();
        assert_eq!(value, 18446744073709551616);
    }

    #[test]
    fn test_model_value_deserialize_float() {
        let value = serde_json::from_str::<Value>("42.0").unwrap();
        assert_eq!(value, Value::Float(42.0));
    }

    #[test]
    fn test_model_value_deserialize_string() {
        let value = serde_json::from_str::<Value>("\"Hello, World!\"").unwrap();
        assert_eq!(value, Value::String("Hello, World!".to_owned()));
    }

    #[test]
    fn test_model_value_deserialize_array() {
        let value = serde_json::from_str::<Value>("[1, 2, 3]").unwrap();
        assert_eq!(value, Value::Array(vec![Value::UInteger(1), Value::UInteger(2), Value::UInteger(3)]));
    }

    #[test]
    fn test_model_value_deserialize_map() {
        let value = serde_json::from_str::<Value>("{\"a\": 1, \"b\": 2, \"c\": 3}").unwrap();
        let mut map = HashMap::<String, Value>::new();
        map.insert("a".to_owned(), Value::UInteger(1));
        map.insert("b".to_owned(), Value::UInteger(2));
        map.insert("c".to_owned(), Value::UInteger(3));
        assert_eq!(value, Value::Map(map));
    }
}