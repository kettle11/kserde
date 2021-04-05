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

use crate::{Serialize, Serializer};
impl<'a> Serialize for Value<'a> {
    fn serialize<E: Serializer>(&self, serializer: &mut E) {
        match self {
            Value::Object(values) => values.serialize(serializer),
            Value::Array(values) => values.serialize(serializer),
            Value::String(s0) => serializer.string(s0),
            Value::Number(n) => serializer.f64(*n),
            Value::Boolean(b) => serializer.bool(*b),
            // This is incorrect as it should not have quotes in JSON.
            Value::Null => serializer.string("null"),
        }
    }
}
