use paste::paste;

use crate::{errors::ConfigError, casting_error, unsupported_conversion_error};

use super::Value;

macro_rules! err_convert_none {
    ($type:ty) => {
        Err(ConfigError::ConversionError{
            target: stringify!($type).to_string(),
            value: "none".to_string(),
        })
    };
}

impl TryInto<bool> for Value {
    type Error = ConfigError;

    fn try_into(self) -> Result<bool, Self::Error> {
        match self {
            Value::Boolean(value) => Ok(value),
            Value::Integer(value) => Ok(value != 0),
            Value::LongInteger(value) => Ok(value != 0),
            Value::UInteger(value) => Ok(value != 0),
            Value::ULongInteger(value) => Ok(value != 0),
            Value::Float(value) => Ok(value != 0.0),
            Value::String(value) => match value.to_lowercase().as_str() {
                "true" => Ok(true),
                "false" => Ok(false),
                _ => casting_error!(String, bool, "can't parse string"),
            },
            Value::None => err_convert_none!(bool),
            Value::Array(_) => unsupported_conversion_error!(Array, bool),
            Value::Map(_) => unsupported_conversion_error!(Map, bool),
        }
    }
}

macro_rules! map_number_error {
    ($from:ty, $to:ty) => {
        |_| ConfigError::CastingError {
            from: stringify!($from).to_string(),
            to: stringify!($to).to_string(),
            why: "Number value is out of target bounds".to_string(),
        }
    };
}

macro_rules! to_number {
    ($value:ident, $from:ty, $to:ty) => {
        <$to>::try_from($value).map_err(map_number_error!($from, $to))
    };
}

impl TryInto<i8> for Value {
    type Error = ConfigError;

    fn try_into(self) -> Result<i8, Self::Error> {
        match self {
            Value::Integer(value) => to_number!(value, i64, i8),
            Value::LongInteger(value) => to_number!(value, i128, i8),
            Value::UInteger(value) => to_number!(value, u64, i8),
            Value::ULongInteger(value) => to_number!(value, u128, i8),
            Value::Float(value) => {
                let val = value as i128;
                to_number!(val, f64, i8)
            },
            Value::Boolean(value) => Ok(value as i8),
            Value::String(value) => match value.parse::<i8>() {
                Ok(value) => Ok(value),
                Err(_) => casting_error!(String, i8, "can't parse string"),
            },
            Value::None => err_convert_none!(i8),
            Value::Array(_) => unsupported_conversion_error!(Array, i8),
            Value::Map(_) => unsupported_conversion_error!(Map, i8),
        }
    }
}

impl TryInto<i16> for Value {
    type Error = ConfigError;

    fn try_into(self) -> Result<i16, Self::Error> {
        match self {
            Value::Integer(value) => to_number!(value, i64, i16),
            Value::LongInteger(value) => to_number!(value, i128, i16),
            Value::UInteger(value) => to_number!(value, u64, i16),
            Value::ULongInteger(value) => to_number!(value, u128, i16),
            Value::Float(value) => {
                let val = value as i128;
                to_number!(val, f64, i16)
            },
            Value::Boolean(value) => Ok(value as i16),
            Value::String(value) => match value.parse::<i16>() {
                Ok(value) => Ok(value),
                Err(_) => casting_error!(String, i16, "can't parse string"),
            },
            Value::None => err_convert_none!(i16),
            Value::Array(_) => unsupported_conversion_error!(Array, i16),
            Value::Map(_) => unsupported_conversion_error!(Map, i16),
        }
    }
}

impl TryInto<i32> for Value {
    type Error = ConfigError;

    fn try_into(self) -> Result<i32, Self::Error> {
        match self {
            Value::Integer(value) => to_number!(value, i64, i32),
            Value::LongInteger(value) => to_number!(value, i128, i32),
            Value::UInteger(value) => to_number!(value, u64, i32),
            Value::ULongInteger(value) => to_number!(value, u128, i32),
            Value::Float(value) => {
                let val = value as i128;
                to_number!(val, f64, i32)
            },
            Value::Boolean(value) => Ok(value as i32),
            Value::String(value) => match value.parse::<i32>() {
                Ok(value) => Ok(value),
                Err(_) => casting_error!(String, i32, "can't parse string"),
            },
            Value::None => err_convert_none!(i32),
            Value::Array(_) => unsupported_conversion_error!(Array, i32),
            Value::Map(_) => unsupported_conversion_error!(Map, i32),
        }
    }
}

impl TryInto<i64> for Value {
    type Error = ConfigError;

    fn try_into(self) -> Result<i64, Self::Error> {
        match self {
            Value::Integer(value) => Ok(value),
            Value::LongInteger(value) => to_number!(value, i128, i64),
            Value::UInteger(value) => to_number!(value, u64, i64),
            Value::ULongInteger(value) => to_number!(value, u128, i64),
            Value::Float(value) => Ok(value as i64),
            Value::Boolean(value) => Ok(value as i64),
            Value::String(value) => match value.parse::<i64>() {
                Ok(value) => Ok(value),
                Err(_) => casting_error!(String, i64, "can't parse string"),
            },
            Value::None => err_convert_none!(i64),
            Value::Array(_) => unsupported_conversion_error!(Array, i64),
            Value::Map(_) => unsupported_conversion_error!(Map, i64),
        }
    }
}

impl TryInto<i128> for Value {
    type Error = ConfigError;

    fn try_into(self) -> Result<i128, Self::Error> {
        match self {
            Value::Integer(value) => Ok(value as i128),
            Value::LongInteger(value) => Ok(value),
            Value::UInteger(value) => Ok(value as i128),
            Value::ULongInteger(value) => to_number!(value, u128, i128),
            Value::Float(value) => Ok(value as i128),
            Value::Boolean(value) => Ok(value as i128),
            Value::String(value) => match value.parse::<i128>() {
                Ok(value) => Ok(value),
                Err(_) => casting_error!(String, i128, "can't parse string"),
            },
            Value::None => err_convert_none!(i128),
            Value::Array(_) => unsupported_conversion_error!(Array, i128),
            Value::Map(_) => unsupported_conversion_error!(Map, i128),
        }
    }
}

impl TryInto<u8> for Value {
    type Error = ConfigError;

    fn try_into(self) -> Result<u8, Self::Error> {
        match self {
            Value::Integer(value) => to_number!(value, i64, u8),
            Value::LongInteger(value) => to_number!(value, i128, u8),
            Value::UInteger(value) => to_number!(value, u64, u8),
            Value::ULongInteger(value) => to_number!(value, u128, u8),
            Value::Float(value) => {
                let val = value as i128;
                to_number!(val, f64, u8)
            },
            Value::Boolean(value) => Ok(value as u8),
            Value::String(value) => match value.parse::<u8>() {
                Ok(value) => Ok(value),
                Err(_) => casting_error!(String, u8, "can't parse string"),
            },
            Value::None => err_convert_none!(u8),
            Value::Array(_) => unsupported_conversion_error!(Array, u8),
            Value::Map(_) => unsupported_conversion_error!(Map, u8),
        }
    }
}

impl TryInto<u16> for Value {
    type Error = ConfigError;

    fn try_into(self) -> Result<u16, Self::Error> {
        match self {
            Value::Integer(value) => to_number!(value, i64, u16),
            Value::LongInteger(value) => to_number!(value, i128, u16),
            Value::UInteger(value) => to_number!(value, u64, u16),
            Value::ULongInteger(value) => to_number!(value, u128, u16),
            Value::Float(value) => {
                let val = value as i128;
                to_number!(val, f64, u16)
            },
            Value::Boolean(value) => Ok(value as u16),
            Value::String(value) => match value.parse::<u16>() {
                Ok(value) => Ok(value),
                Err(_) => casting_error!(String, u16, "can't parse string"),
            },
            Value::None => err_convert_none!(u16),
            Value::Array(_) => unsupported_conversion_error!(Array, u16),
            Value::Map(_) => unsupported_conversion_error!(Map, u16),
        }
    }
}

impl TryInto<u32> for Value {
    type Error = ConfigError;

    fn try_into(self) -> Result<u32, Self::Error> {
        match self {
            Value::Integer(value) => to_number!(value, i64, u32),
            Value::LongInteger(value) => to_number!(value, i128, u32),
            Value::UInteger(value) => to_number!(value, u64, u32),
            Value::ULongInteger(value) => to_number!(value, u128, u32),
            Value::Float(value) => {
                let val = value as i128;
                to_number!(val, f64, u32)
            },
            Value::Boolean(value) => Ok(value as u32),
            Value::String(value) => match value.parse::<u32>() {
                Ok(value) => Ok(value),
                Err(_) => casting_error!(String, u32, "can't parse string"),
            },
            Value::None => err_convert_none!(u32),
            Value::Array(_) => unsupported_conversion_error!(Array, u32),
            Value::Map(_) => unsupported_conversion_error!(Map, u32),
        }
    }
}

impl TryInto<u64> for Value {
    type Error = ConfigError;

    fn try_into(self) -> Result<u64, Self::Error> {
        match self {
            Value::Integer(value) => to_number!(value, i64, u64),
            Value::LongInteger(value) => to_number!(value, i128, u64),
            Value::UInteger(value) => Ok(value),
            Value::ULongInteger(value) => to_number!(value, u128, u64),
            Value::Float(value) => Ok(value as u64),
            Value::Boolean(value) => Ok(value as u64),
            Value::String(value) => match value.parse::<u64>() {
                Ok(value) => Ok(value),
                Err(_) => casting_error!(String, u64, "can't parse string"),
            },
            Value::None => err_convert_none!(u64),
            Value::Array(_) => unsupported_conversion_error!(Array, u64),
            Value::Map(_) => unsupported_conversion_error!(Map, u64),
        }
    }
}

impl TryInto<u128> for Value {
    type Error = ConfigError;

    fn try_into(self) -> Result<u128, Self::Error> {
        match self {
            Value::Integer(value) => to_number!(value, i64, u128),
            Value::LongInteger(value) => to_number!(value, i128, u128),
            Value::UInteger(value) => Ok(value as u128),
            Value::ULongInteger(value) => Ok(value),
            Value::Float(value) => {
                let val = value as i128;
                to_number!(val, f64, u128)
            },
            Value::Boolean(value) => Ok(value as u128),
            Value::String(value) => match value.parse::<u128>() {
                Ok(value) => Ok(value),
                Err(_) => casting_error!(String, u128, "can't parse string"),
            },
            Value::None => err_convert_none!(u128),
            Value::Array(_) => unsupported_conversion_error!(Array, u128),
            Value::Map(_) => unsupported_conversion_error!(Map, u128),
        }
    }
}

impl TryInto<f64> for Value {
    type Error = ConfigError;

    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            Value::Float(value) => Ok(value),
            Value::Integer(value) => Ok(value as f64),
            Value::LongInteger(value) => Ok(value as f64),
            Value::UInteger(value) => Ok(value as f64),
            Value::ULongInteger(value) => Ok(value as f64),
            Value::Boolean(value) => Ok(value as i64 as f64),
            Value::String(value) => match value.parse::<f64>() {
                Ok(value) => Ok(value),
                Err(_) => casting_error!(String, f64, "can't parse string"),
            },
            Value::None => err_convert_none!(f64),
            Value::Array(_) => unsupported_conversion_error!(Array, f64),
            Value::Map(_) => unsupported_conversion_error!(Map, f64),
        }
    }
}

impl TryInto<char> for Value {
    type Error = ConfigError;

    fn try_into(self) -> Result<char, Self::Error> {
        match self {
            Value::String(value) => match value.chars().next() {
                Some(value) => Ok(value),
                None => casting_error!(String, char, "can't parse string"),
            },
            Value::None => err_convert_none!(char),
            Value::Boolean(value) => Ok(if value { '1' } else { '0' }),
            Value::Integer(value) => {
                match to_number!(value, i64, u8) {
                    Ok(value) => Ok(value as char),
                    Err(err) => Err(err),
                }
            },
            Value::LongInteger(value) => {
                match to_number!(value, i128, u8) {
                    Ok(value) => Ok(value as char),
                    Err(err) => Err(err),
                }
            },
            Value::UInteger(value) => {
                match to_number!(value, u64, u8) {
                    Ok(value) => Ok(value as char),
                    Err(err) => Err(err),
                }
            },
            Value::ULongInteger(value) => {
                match to_number!(value, i64, u8) {
                    Ok(value) => Ok(value as char),
                    Err(err) => Err(err),
                }
            },
            Value::Float(_) => unsupported_conversion_error!(Float, char),
            Value::Array(_) => unsupported_conversion_error!(Array, char),
            Value::Map(_) => unsupported_conversion_error!(Map, char),
        }
    }
}

impl TryInto<String> for Value {
    type Error = ConfigError;

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            Value::String(value) => Ok(value),
            Value::Integer(value) => Ok(value.to_string()),
            Value::LongInteger(value) => Ok(value.to_string()),
            Value::Float(value) => Ok(value.to_string()),
            Value::UInteger(value) => Ok(value.to_string()),
            Value::ULongInteger(value) => Ok(value.to_string()),
            Value::Boolean(value) => Ok(value.to_string()),
            Value::None => err_convert_none!(String),
            Value::Array(_) => unsupported_conversion_error!(Array, String),
            Value::Map(_) => unsupported_conversion_error!(Map, String),
        }
    }
}

macro_rules! try_into_ref {
    ($type:ty) => {
        impl TryInto<$type> for &Value {
            type Error = ConfigError;
        
            fn try_into(self) -> Result<$type, Self::Error> {
                self.to_owned().try_into()
            }
        }
    };
}

macro_rules! try_into_mut {
    ($type:ty) => {
        impl TryInto<$type> for &mut Value {
            type Error = ConfigError;
        
            fn try_into(self) -> Result<$type, Self::Error> {
                self.to_owned().try_into()
            }
        }
    };
}

macro_rules! try_into_types {
    ($($type:ty),* $(,)?) => {
        $(
            try_into_ref!($type);
            try_into_mut!($type);
        )*

        impl Value {
            $(
                paste! {
                    pub fn [<try_into_ $type:lower>](&self) -> Result<$type, ConfigError> {
                        self.try_into()
                    }

                    pub fn [<into_ $type:lower _or>](self, default: $type) -> $type {
                        self.try_into().unwrap_or(default)
                    }
                }
            )*
        }
    };
}

try_into_types!(
    bool, f64, char, String,
    i8, i16, i32, i64, i128,
    u8, u16, u32, u64, u128,
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_value_try_into_i64() {
        let value = Value::LongInteger(100);
        assert_eq!(value.try_into(), Ok(100 as i64));

        let value: Value = 1.0.into();
        assert_eq!(value.try_into(), Ok(1 as i64));

        let value: Value = true.into();
        assert_eq!(value.try_into(), Ok(1 as i64));

        let value: Value = "1".into();
        assert_eq!(value.try_into(), Ok(1 as i64));

        let value = Value::LongInteger(i128::MAX);
        let res: Result<i64, ConfigError> = value.try_into();
        assert_eq!(res.unwrap_err(), ConfigError::CastingError {
            from: "i128".to_string(),
            to: "i64".to_string(),
            why: "Number value is out of target bounds".to_string(),
        });
    }

    #[test]
    fn test_model_value_try_into_i128() {
        let value = Value::LongInteger(100);
        assert_eq!(value.try_into(), Ok(100 as i128));

        let value: Value = 1.0.into();
        assert_eq!(value.try_into(), Ok(1 as i128));

        let value: Value = true.into();
        assert_eq!(value.try_into(), Ok(1 as i128));

        let value: Value = "1".into();
        assert_eq!(value.try_into(), Ok(1 as i128));

        let value = Value::LongInteger(i128::MAX);
        assert_eq!(value.try_into(), Ok(i128::MAX));

        let value = Value::ULongInteger(u128::MAX);
        let res: Result<i128, ConfigError> = value.try_into();
        assert_eq!(res.unwrap_err(), ConfigError::CastingError {
            from: "u128".to_string(),
            to: "i128".to_string(),
            why: "Number value is out of target bounds".to_string(),
        });
    }

    #[test]
    fn test_model_value_try_into_u64() {
        let value = Value::UInteger(100);
        assert_eq!(value.try_into(), Ok(100 as u64));

        let value: Value = 1.0.into();
        assert_eq!(value.try_into(), Ok(1 as u64));

        let value: Value = true.into();
        assert_eq!(value.try_into(), Ok(1 as u64));

        let value: Value = "1".into();
        assert_eq!(value.try_into(), Ok(1 as u64));

        let value = Value::ULongInteger(u128::MAX);
        let res: Result<u64, ConfigError> = value.try_into();
        assert_eq!(res.unwrap_err(), ConfigError::CastingError {
            from: "u128".to_string(),
            to: "u64".to_string(),
            why: "Number value is out of target bounds".to_string(),
        });

        let value = Value::LongInteger(-1);
        let res: Result<u64, ConfigError> = value.try_into();
        assert_eq!(res.unwrap_err(), ConfigError::CastingError {
            from: "i128".to_string(),
            to: "u64".to_string(),
            why: "Number value is out of target bounds".to_string(),
        });
    }

    #[test]
    fn test_model_value_try_into_u128() {
        let value = Value::UInteger(100);
        assert_eq!(value.try_into(), Ok(100 as u128));

        let value: Value = 1.0.into();
        assert_eq!(value.try_into(), Ok(1 as u128));

        let value: Value = true.into();
        assert_eq!(value.try_into(), Ok(1 as u128));

        let value: Value = "1".into();
        assert_eq!(value.try_into(), Ok(1 as u128));

        let value = Value::ULongInteger(u128::MAX);
        assert_eq!(value.try_into(), Ok(u128::MAX));

        let value = Value::LongInteger(-1);
        let res: Result<u128, ConfigError> = value.try_into();
        assert_eq!(res.unwrap_err(), ConfigError::CastingError {
            from: "i128".to_string(),
            to: "u128".to_string(),
            why: "Number value is out of target bounds".to_string(),
        });
    }

    #[test]
    fn test_model_value_try_into_f64() {
        let value = Value::Float(100.0);
        assert_eq!(value.try_into(), Ok(100.0 as f64));

        let value: Value = 1.0.into();
        assert_eq!(value.try_into(), Ok(1.0 as f64));

        let value: Value = true.into();
        assert_eq!(value.try_into(), Ok(1.0 as f64));

        let value: Value = "1".into();
        assert_eq!(value.try_into(), Ok(1.0 as f64));
    }

    #[test]
    fn test_model_value_try_into_string() {
        let value: Value = "test".into();
        assert_eq!(value.try_into(), Ok("test".to_string()));

        let value: Value = 1.into();
        assert_eq!(value.try_into(), Ok("1".to_string()));

        let value: Value = 1.0.into();
        assert_eq!(value.try_into(), Ok("1".to_string()));

        let value: Value = true.into();
        assert_eq!(value.try_into(), Ok("true".to_string()));
    }
}