mod from_str;
mod to_string;
pub use from_str::*;
pub use to_string::*;

use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value<'a> {
    String(Cow<'a, str>),
    Number(f64),
    Object(HashMap<Cow<'a, str>, Value<'a>>),
    Array(Vec<Value<'a>>),
    Boolean(bool),
    Null,
}

impl<'a> Value<'a> {
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Value::String(a) => Some(&a),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match *self {
            Value::Number(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&HashMap<Cow<'a, str>, Value<'a>>> {
        match self {
            Value::Object(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<Value<'a>>> {
        match self {
            Value::Array(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        match *self {
            Value::Boolean(b) => Some(b),
            _ => None,
        }
    }
}
