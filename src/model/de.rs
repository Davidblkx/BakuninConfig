use std::{iter::Enumerate, vec::IntoIter, collections::{VecDeque, HashMap}};

use crate::errors::ConfigError;

use super::Value;

impl<'de> serde::de::Deserializer<'de> for Value {
    type Error = ConfigError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        match self {
            Value::None => visitor.visit_unit(),
            Value::Boolean(value) => visitor.visit_bool(value),
            Value::Integer(value) => visitor.visit_i64(value),
            Value::LongInteger(value) => visitor.visit_i128(value),
            Value::UInteger(value) => visitor.visit_u64(value),
            Value::ULongInteger(value) => visitor.visit_u128(value),
            Value::Float(value) => visitor.visit_f64(value),
            Value::String(value) => visitor.visit_str(&value),
            Value::Array(value) => visitor.visit_seq(ValueSeq::new(value)),
            Value::Map(value) => visitor.visit_map(ValueMap::new(value)),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
            visitor.visit_bool(self.try_into()?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
            visitor.visit_i8(self.try_into()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
            visitor.visit_i16(self.try_into()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
            visitor.visit_i32(self.try_into()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
            visitor.visit_i64(self.try_into()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
            visitor.visit_u8(self.try_into()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
            visitor.visit_u16(self.try_into()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
            visitor.visit_u32(self.try_into()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
            visitor.visit_u64(self.try_into()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
            let v: f64 = self.try_into()?;
            visitor.visit_f32(v as f32)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
            visitor.visit_f64(self.try_into()?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
            visitor.visit_char(self.try_into()?)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
            let val: String = self.try_into()?;
            visitor.visit_str(&val)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
            visitor.visit_string(self.try_into()?)
    }

    fn deserialize_bytes<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
            unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
            unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
            match self {
                Value::None => visitor.visit_none(),
                _ => visitor.visit_some(self),
            }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
            match self {
                Value::None => visitor.visit_unit(),
                _ => Err(ConfigError::ConversionError {
                    target: "none".to_string(),
                    value: self.to_string(),
                }),
            }
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
            self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
            visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
            match self {
                Value::Array(value) => visitor.visit_seq(ValueSeq::new(value)),
                _ => Err(ConfigError::ConversionError {
                    target: "array".to_string(),
                    value: self.to_string(),
                }),
            }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
            self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
            self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
            match self {
                Value::Map(value) => visitor.visit_map(ValueMap::new(value)),
                _ => Err(ConfigError::ConversionError {
                    target: "map".to_string(),
                    value: self.to_string(),
                }),
            }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
            self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
            unimplemented!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
            self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
            self.deserialize_any(visitor)
    }
}

struct ValueSeq {
    elements: Enumerate<IntoIter<Value>>,
}

impl ValueSeq {
    pub fn new(elements: Vec<Value>) -> Self {
        Self {
            elements: elements.into_iter().enumerate(),
        }
    }
}

impl<'de> serde::de::SeqAccess<'de> for ValueSeq {
    type Error = ConfigError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de> {
        match self.elements.next() {
            None => Ok(None),
            Some((_, value)) => seed
                .deserialize(value)
                .map(Some)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        match self.elements.size_hint() {
            (lower, Some(upper)) if lower == upper => Some(upper),
            _ => None,
        }
    }
}

struct ValueMap {
    elements: VecDeque<(String, Value)>,
}

impl ValueMap {
    pub fn new(object: HashMap<String, Value>) -> Self {
        Self {
            elements: object.into_iter().collect(),
        }
    }
}

impl<'de> serde::de::MapAccess<'de> for ValueMap {
    type Error = ConfigError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de> {
        if let Some((ref key, _)) = self.elements.front() {
            let value = Value::String(key.to_string());
            Ok(Some(serde::de::DeserializeSeed::deserialize(seed, value)?))
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de> {
            let (_, value) = self.elements.pop_front().unwrap();
            serde::de::DeserializeSeed::deserialize(seed, value)
    }
}

impl Value {
    pub fn deserialize<'de, T>(self) -> Result<T, ConfigError>
        where
            T: serde::de::Deserialize<'de>, {
        T::deserialize(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::value_map;

    use serde::Deserialize;

    #[derive(Deserialize)]
    struct Address {
        pub street: String,
        pub city: String,
    }

    #[derive(Deserialize)]
    struct TestPerson {
        pub name: String,
        pub age: u8,
        pub address: Address,
    }

    #[test]
    fn test_deserialize() {
        let map = value_map! {
            name: "John",
            age: 42,
            address: value_map! {
                street: "10 Downing Street",
                city: "London",
            },
        };

        let person = map.deserialize::<TestPerson>().unwrap();
        assert_eq!(person.name, "John");
        assert_eq!(person.age, 42);
        assert_eq!(person.address.street, "10 Downing Street");
        assert_eq!(person.address.city, "London");
    }
}
