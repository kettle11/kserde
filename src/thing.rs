use crate::serialize_trait::ArraySerializer;
use crate::serialize_trait::ObjectSerializer;
use crate::{AnyValue, Deserialize, Deserializer, JSONDeserializer, Serialize, Serializer};
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ObjectProperty<'a> {
    pub item: Thing<'a>,
    pub index: usize,
}

#[derive(Debug)]
/// A flexible data structure that everything can deserialize to.
pub enum Thing<'a> {
    String(Cow<'a, str>),
    Bool(bool),
    Number(f64),
    Object(HashMap<Cow<'a, str>, ObjectProperty<'a>>),
    Array(Vec<Thing<'a>>),
    Null,
}

impl<'a> Thing<'a> {
    pub fn string(&self) -> Option<&Cow<'a, str>> {
        match self {
            Thing::String(v) => Some(v),
            _ => None,
        }
    }

    pub fn bool(&self) -> Option<bool> {
        match self {
            Thing::Bool(v) => Some(*v),
            _ => None,
        }
    }

    pub fn number(&self) -> Option<f64> {
        match self {
            Thing::Number(v) => Some(*v),
            _ => None,
        }
    }

    pub fn object(&self) -> Option<&HashMap<Cow<'a, str>, ObjectProperty<'a>>> {
        match self {
            Thing::Object(v) => Some(v),
            _ => None,
        }
    }

    pub fn array(&self) -> Option<&Vec<Thing<'a>>> {
        match self {
            Thing::Array(v) => Some(v),
            _ => None,
        }
    }
}

impl<'a> Deserialize<'a> for Thing<'a> {
    fn deserialize<D: Deserializer<'a>>(deserializer: &mut D) -> Option<Self> {
        Some(match deserializer.any()? {
            AnyValue::Object => {
                let mut items = HashMap::new();
                while let Some(name) = deserializer.has_property() {
                    items.insert(
                        name,
                        ObjectProperty {
                            index: items.len(),
                            item: Thing::deserialize(deserializer)?,
                        },
                    );
                }
                Thing::Object(items)
            }
            AnyValue::Array => {
                let mut items = Vec::new();
                while deserializer.has_array_value() {
                    items.push(Thing::deserialize(deserializer)?);
                }
                Thing::Array(items)
            }
            AnyValue::Number(n) => Thing::Number(n),
            AnyValue::Bool(b) => Thing::Bool(b),
            AnyValue::String(s) => Thing::String(s),
            AnyValue::Null => Thing::Null,
        })
    }
}

impl<'a> Serialize for Thing<'a> {
    fn serialize<S: Serializer>(&self, serializer: &mut S) {
        match self {
            Self::Object(o) => {
                let mut object = serializer.begin_object();
                // This allocation and sorting probably isn't ideal,
                // The alternative is to use a sorted HashMap when deserializing
                // into Thing.
                let mut properties: Vec<_> = o.iter().collect();
                properties.sort_by_key(|(_, i)| i.index);
                for (key, value) in o.iter() {
                    object.property(key, &value.item);
                }
                object.end_object();
            }
            Self::Array(a) => {
                let mut array = serializer.begin_array();
                for value in a.iter() {
                    array.value(value);
                }
                array.end_array();
            }
            Self::Number(n) => serializer.f64(*n),
            Self::Bool(b) => serializer.bool(*b),
            Self::String(s) => serializer.string(&s),
            Self::Null => serializer.null(),
        }
    }
}

impl<'a> Thing<'a> {
    pub fn from_json(s: &'a str) -> Option<Self> {
        let mut deserializer = JSONDeserializer::new(s);
        Self::deserialize(&mut deserializer)
    }
}
