use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModelError {
    #[error("Operation only supported for map Value")]
    OperationOnlyForMapValue,
    #[error("Operation only supported for array Value")]
    OperationOnlyForArrayValue,
    #[error("Error serializing from value: {0}")]
    SerializationError(String),
    #[error("Error deserializing to value: {0}")]
    DeserializationError(String),
    #[error("Error converting from {from:?} to {to:?}: {why:?}")]
    ConversionError {
        from: &'static str,
        to: &'static str,
        why: &'static str,
    },
}

impl serde::de::Error for ModelError {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Self::DeserializationError(msg.to_string())
    }
}

impl serde::ser::Error for ModelError {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Self::SerializationError(msg.to_string())
    }
}

impl ModelError {
    pub fn to_result<T>(self) -> Result<T, Self> {
        Err(self)
    }

    pub fn deserialization(from: String, to: &'static str) -> Self {
        Self::DeserializationError(format!("Can't deserialize {from} to {to}"))
    }
}

impl PartialEq for ModelError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ModelError::OperationOnlyForMapValue, ModelError::OperationOnlyForMapValue) => true,
            (ModelError::OperationOnlyForArrayValue, ModelError::OperationOnlyForArrayValue) => {
                true
            }
            (ModelError::SerializationError(msg1), ModelError::SerializationError(msg2)) => {
                msg1 == msg2
            }
            (ModelError::DeserializationError(msg1), ModelError::DeserializationError(msg2)) => {
                msg1 == msg2
            }
            (
                ModelError::ConversionError {
                    from: f1,
                    to: t1,
                    why: w1,
                },
                ModelError::ConversionError {
                    from: f2,
                    to: t2,
                    why: w2,
                },
            ) => f1 == f2 && t1 == t2 && w1 == w2,
            _ => false,
        }
    }
}

macro_rules! conversion_error {
    ($from: ident, $to: ident) => {
        $crate::model::ModelError::ConversionError {
            from: stringify!($from),
            to: stringify!($to),
            why: "conversion not supported",
        }
    };
    ($from: ident, $to: ident, $why: literal) => {
        $crate::model::ModelError::ConversionError {
            from: stringify!($from),
            to: stringify!($to),
            why: $why,
        }
    };
}

macro_rules! err_conversion_error {
    ($from: ident, $to: ident) => {
        Err(conversion_error!($from, $to))
    };
    ($from: ident, $to: ident, $why: literal) => {
        Err(conversion_error!($from, $to, $why))
    };
}
