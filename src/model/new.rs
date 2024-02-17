use std::collections::HashMap;

use super::Value;

impl Value {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_map() -> Self {
        Self::Map(HashMap::new())
    }

    pub fn new_array() -> Self {
        Self::Array(Vec::new())
    }

    pub fn with_array_capacity(size: usize) -> Self {
        Self::Array(Vec::with_capacity(size))
    }
}

#[macro_export]
macro_rules! value_map {
    ($($key:ident: $value:expr),* $(,)?) => {
        {
            let mut map = crate::Value::new_map();
            $(
                map.set(stringify!($key), $value.into()).unwrap();
            )*
            map
        }
    };
}

#[macro_export]
macro_rules! value_vec {
    ($($value:expr),* $(,)?) => {
        {
            let mut array = Value::new_array();
            $(
                array.push($value.into()).unwrap();
            )*
            array
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_value_new_map() {
        let val = Value::new_map()
            .set("a", 1.into()).unwrap()
            .set("b", "data".into()).unwrap()
            .to_owned();

        assert_eq!(val.get("a").try_into_i64().unwrap(), 1);
        assert_eq!(val.get("b").try_into_string().unwrap(), "data");
    }

    #[test]
    fn test_model_value_new_array() {
        let val = value_vec![1, "data", 3.14, true];

        assert_eq!(val.len(), 4);
        assert_eq!(val.at(0).try_into_i64().unwrap(), 1);
        assert_eq!(val.at(1).try_into_string().unwrap(), "data");
        assert_eq!(val.at(2).try_into_f64().unwrap(), 3.14);
        assert_eq!(val.at(3).try_into_bool().unwrap(), true);
    }

    #[test]
    fn test_model_value_map_macro() {
        let val = value_map! {
            a: 1,
            b: "data",
            c: 3.14,
            d: true,
            e: value_map! {
                a: 1,
                b: "data",
                c: 3.14,
                d: true,
            },
            f: value_vec![1, "data", 3.14, true],
            g: Option::None::<i64>,
        };

        assert_eq!(val.get("a").try_into_i64().unwrap(), 1);
        assert_eq!(val.get("b").try_into_string().unwrap(), "data");
        assert_eq!(val.get("c").try_into_f64().unwrap(), 3.14);
        assert_eq!(val.get("d").try_into_bool().unwrap(), true);
        assert_eq!(val.get("e").get("a").try_into_i64().unwrap(), 1);
        assert_eq!(val.get("e").get("b").try_into_string().unwrap(), "data");
        assert_eq!(val.get("e").get("c").try_into_f64().unwrap(), 3.14);
        assert_eq!(val.get("e").get("d").try_into_bool().unwrap(), true);
        assert_eq!(val.get("f").at(0).try_into_i64().unwrap(), 1);
        assert_eq!(val.get("f").at(1).try_into_string().unwrap(), "data");
        assert_eq!(val.get("f").at(2).try_into_f64().unwrap(), 3.14);
        assert_eq!(val.get("f").at(3).try_into_bool().unwrap(), true);
        assert!(val.get("g").is_none());
    }
}