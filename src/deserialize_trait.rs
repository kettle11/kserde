use std::borrow::Cow;

/// If a value returns `None` the deserializer will still be incremented
/// and the deserializer will no longer be in a valid state.
pub trait Deserializer<'a> {
    type ObjectDeserializer: ObjectDeserializer<'a>;

    fn string(&mut self) -> Option<Cow<'a, str>>;
    fn bool(&mut self) -> Option<bool>;
    fn i64(&mut self) -> Option<i64>;
    fn f64(&mut self) -> Option<f64>;
    fn any(&mut self) -> Option<AnyValue<'a, Self::ObjectDeserializer>>;

    fn begin_object(&mut self) -> Option<Self::ObjectDeserializer>;
}

//pub trait Deserializer<'a: 'b, 'b>: DeserializeValue<'a> + AsObjectDeserializer<'a, 'b> {}

pub trait Deserialize: Sized {
    fn deserialize<'a, D: Deserializer<'a>>(deserializer: &mut D) -> Option<Self>;
}

pub trait ObjectDeserializer<'a> {
    type Deserializer: Deserializer<'a>;
    /// Returns the property name and a deserializer that can be used to deserialize the value.
    fn property(&mut self) -> Option<(Cow<'a, str>, Self::Deserializer)>;
    fn end_object(self) -> Option<()>;
}

/*
pub trait ArrayDeserializer<'a> {
    type Deserializer: for<'b> Deserializer<'b>;
    /// Returns the property name and a deserializer that can be used to deserialize the
    /// value.
    fn property(&mut self) -> (Cow<'a, str>, Self::Deserializer);
}
*/

pub enum AnyValue<'a, O: ObjectDeserializer<'a>> {
    String(std::borrow::Cow<'a, str>),
    Bool(bool),
    Number(f64),
    Object(O),
    Null,
}

impl<'a, O: ObjectDeserializer<'a>> AnyValue<'a, O> {
    pub fn string(self) -> Option<Cow<'a, str>> {
        match self {
            Self::String(a) => Some(a),
            _ => None,
        }
    }

    pub fn number(self) -> Option<f64> {
        match self {
            Self::Number(v) => Some(v),
            _ => None,
        }
    }

    pub fn object(self) -> Option<O> {
        match self {
            Self::Object(v) => Some(v),
            _ => None,
        }
    }

    pub fn boolean(self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(b),
            _ => None,
        }
    }
}
