use serde::ser::{Serializer, SerializeSeq, SerializeMap};

use super::Value;

use crate::errors::ConfigError;

impl serde::ser::Serialize for Value {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {
        match self {
            Value::None => s.serialize_none(),
            Value::Boolean(value) => s.serialize_bool(*value),
            Value::Integer(value) => s.serialize_i64(*value),
            Value::LongInteger(value) => s.serialize_i128(*value),
            Value::UInteger(value) => s.serialize_u64(*value),
            Value::ULongInteger(value) => s.serialize_u128(*value),
            Value::Float(value) => s.serialize_f64(*value),
            Value::String(value) => s.serialize_str(value),
            Value::Array(value) => {
                let mut seq = s.serialize_seq(Some(value.len()))?;
                for v in value {
                    seq.serialize_element(v)?;
                }
                seq.end()
            }
            Value::Map(value) => {
                let mut map = s.serialize_map(Some(value.len()))?;
                for (k, v) in value {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
        }
    }
}

impl Value {
    pub fn serialize<T>(value: T) -> Result<Value, ConfigError>
    where
        T: serde::Serialize {
            value.serialize(ValueSerializer)
    }
}

pub struct ValueSerializer;

impl serde::Serializer for ValueSerializer {
    type Ok = Value;

    type Error = ConfigError;

    type SerializeSeq = ValueSerSeq;

    type SerializeTuple = ValueSerSeq;

    type SerializeTupleStruct = ValueSerSeq;

    type SerializeTupleVariant = ValueSerTupleVariant;

    type SerializeMap = ValueSerMap;

    type SerializeStruct = ValueSerMap;

    type SerializeStructVariant = ValueSerStructVariant;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(v))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(v.to_string()))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(v))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        let mut value = Value::new_array();
        v.iter().for_each(|e| {
            value.push(Value::UInteger(e.to_owned().into())).unwrap();
        });
        Ok(value)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::None)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize {
            value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_none()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_none()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize {
        let mut res = Value::new_map();
        res.set(variant, value.serialize(ValueSerializer)?)?;
        Ok(res)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(ValueSerSeq::new(len.unwrap_or(0)))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(ValueSerTupleVariant::new(variant.to_string(), len))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(ValueSerMap::new())
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(ValueSerStructVariant::new(variant.to_string()))
    }
}

pub struct ValueSerSeq {
    seq: Value,
}

impl ValueSerSeq {
    pub fn new(size: usize) -> Self {
        Self {
            seq: Value::with_array_capacity(size),
        }
    }
}

macro_rules! impl_ser_seq {
    ($fn_name:ident) => {
        type Ok = Value;

        type Error = ConfigError;

        fn $fn_name<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: serde::Serialize {
            let elem = value.serialize(ValueSerializer)?;
            self.seq.push(elem)?;
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(self.seq)
        }
    };
}

impl serde::ser::SerializeSeq for ValueSerSeq {
    impl_ser_seq!(serialize_element);
}

impl serde::ser::SerializeTuple for ValueSerSeq {
    impl_ser_seq!(serialize_element); 
}

impl serde::ser::SerializeTupleStruct for ValueSerSeq {
    impl_ser_seq!(serialize_field);
}

pub struct ValueSerTupleVariant {
    seq: Value,
    name: String,
}

impl ValueSerTupleVariant {
    pub fn new(name: String, size: usize) -> Self {
        Self {
            seq: Value::with_array_capacity(size),
            name,
        }
    }
}

impl serde::ser::SerializeTupleVariant for ValueSerTupleVariant {
    type Ok = Value;

    type Error = ConfigError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
        let elem = value.serialize(ValueSerializer)?;
        self.seq.push(elem)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let mut value = Value::new_map();
        value.set(&self.name, self.seq)?;
        Ok(value)
    }
}

pub struct ValueSerMap {
    map: Value,
    key: Option<String>,
}

impl ValueSerMap {
    pub fn new() -> Self {
        Self {
            map: Value::new_map(),
            key: None,
        }
    }
}

impl serde::ser::SerializeMap for ValueSerMap {
    type Ok = Value;

    type Error = ConfigError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
        let key = key.serialize(ValueSerializer)?;
        self.key = Some(key.try_into_string()?);
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
            let key = self.key.take().expect("serialize_value called before serialize_key");
            let value = value.serialize(ValueSerializer)?;
            self.map.set(&key, value)?;
            Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.map)
    }
}

pub struct ValueSerStructVariant {
    map: Value,
    name: String,
}

impl ValueSerStructVariant {
    pub fn new(name: String) -> Self {
        Self {
            map: Value::new_map(),
            name,
        }
    }
}

impl serde::ser::SerializeStructVariant for ValueSerStructVariant {
    type Ok = Value;

    type Error = ConfigError;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
        let value = value.serialize(ValueSerializer)?;
        self.map.set(&key, value)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let mut value = Value::new_map();
        value.set(&self.name, self.map)?;
        Ok(value)
    }
}

impl serde::ser::SerializeStruct for ValueSerMap {
    type Ok = Value;

    type Error = ConfigError;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
        let value = value.serialize(ValueSerializer)?;
        self.map.set(&key, value)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.map)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde::Serialize;

    use super::*;

    #[derive(Serialize)]
    struct TestStruct {
        name: String,
        age: i64,
        childs: Vec<TestStruct>,
        fam: HashMap<String, TestStruct>,
        data: Option<u64>,
    }

    impl TestStruct {
        pub fn new(name: &str, age: i64) -> Self {
            TestStruct {
                name: name.to_string(),
                age,
                childs: vec![],
                fam: HashMap::new(),
                data: Some(10)
            }
        }
    }

    #[test]
    fn test_model_value_serialize_none() {
        let value = Value::None;
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, "null");
    }

    #[test]
    fn test_model_value_serialize_boolean() {
        let value = Value::Boolean(true);
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, "true");
    }

    #[test]
    fn test_model_value_serialize_integer() {
        let value = Value::Integer(42);
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, "42");
    }

    #[test]
    fn test_model_value_serialize_long_integer() {
        let value = Value::LongInteger(42);
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, "42");
    }

    #[test]
    fn test_model_value_serialize_uinteger() {
        let value = Value::UInteger(42);
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, "42");
    }

    #[test]
    fn test_model_value_serialize_ulong_integer() {
        let value = Value::ULongInteger(42);
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, "42");
    }

    #[test]
    fn test_model_value_serialize_float() {
        let value = Value::Float(42.0);
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, "42.0");
    }

    #[test]
    fn test_model_value_serialize_string() {
        let value = Value::String("42".to_owned());
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, "\"42\"");
    }

    #[test]
    fn test_model_value_serialize_array() {
        let value = Value::Array(vec![Value::Integer(42), Value::Integer(43)]);
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, "[42,43]");
    }

    #[test]
    fn test_model_value_serialize_map() {
        let hmap = {
            let mut hmap = std::collections::HashMap::new();
            hmap.insert("a".to_owned(), Value::Integer(42));
            hmap
        };
        let value = Value::Map(hmap);
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, "{\"a\":42}");
    }

    #[test]
    fn test_model_value_serializer() {
        let mut user = TestStruct::new("John", 42);
        
        let child1 = TestStruct::new("Jane", 12);
        user.childs.push(child1);
        let child2 = TestStruct::new("Jack", 10);
        user.childs.push(child2);

        let wife = TestStruct::new("Jane", 45);
        user.fam.insert("wife".to_owned(), wife);

        let cousin = TestStruct::new("Jack", 51);
        user.fam.insert("cousin".to_owned(), cousin);

        let val = Value::serialize(user).unwrap();

        assert_eq!(val.get("name").try_into_string().unwrap(), "John");
        assert_eq!(val.get("age").try_into_i64().unwrap(), 42);
        assert_eq!(val.get("childs").at(0).get("name").try_into_string().unwrap(), "Jane");
        assert_eq!(val.get("childs").at(0).get("age").try_into_i64().unwrap(), 12);
        assert_eq!(val.get("childs").at(1).get("name").try_into_string().unwrap(), "Jack");
        assert_eq!(val.get("childs").at(1).get("age").try_into_i64().unwrap(), 10);
        assert_eq!(val.get("fam").get("wife").get("name").try_into_string().unwrap(), "Jane");
        assert_eq!(val.get("fam").get("wife").get("age").try_into_i64().unwrap(), 45);
        assert_eq!(val.get("fam").get("cousin").get("name").try_into_string().unwrap(), "Jack");
        assert_eq!(val.get("fam").get("cousin").get("age").try_into_i64().unwrap(), 51);
    }
}