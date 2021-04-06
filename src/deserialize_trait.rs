use std::borrow::Cow;

/// If a value returns `None` then it should be assumed that the deserializer is
/// no longer in a valid state.
pub trait Deserializer<'a> {
    fn string(&mut self) -> Option<Cow<'a, str>>;
    fn bool(&mut self) -> Option<bool>;
    fn i64(&mut self) -> Option<i64>;
    fn f64(&mut self) -> Option<f64>;
    fn any<'b>(&'b mut self) -> Option<AnyValue<'a>>;

    // I'd prefer the rest of this to be a different trait that
    // borrows from the deserializer, but I couldn't figure out
    // how to make that work without generic associated types,
    // so these functions are here instead.
    fn begin_object(&mut self) -> bool;
    /// When this returns `None` we're at the end of the object or an error was encountered.
    /// The name of the property is returned.
    fn has_property(&mut self) -> Option<Cow<'a, str>>;

    fn begin_array(&mut self) -> bool;
    /// When this returns `None` we're at the end of the array or an error was encountered.
    fn has_array_value(&mut self) -> bool;
}

pub trait Deserialize<'a>: Sized {
    fn deserialize<D: Deserializer<'a>>(deserializer: &mut D) -> Option<Self>;
}

pub enum AnyValue<'a> {
    String(std::borrow::Cow<'a, str>),
    Bool(bool),
    Number(f64),
    Object,
    Array,
    Null,
}

impl<'a> AnyValue<'a> {
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
}
