use std::collections::HashMap;

pub trait Serializer:
    for<'a> AsObjectSerializer<'a> + for<'a> AsArraySerializer<'a> + Sized
{
    type Result;
    fn new() -> Self;
    fn string(&mut self, s: &str);
    fn bool(&mut self, b: bool);
    fn i64(&mut self, i: i64);
    fn f64(&mut self, n: f64);
    fn null(&mut self);

    /// Serialize a value that implements Serialize.
    fn serialize<V: Serialize>(&mut self, value: &V) {
        V::serialize(value, self);
    }
    fn done(self) -> Self::Result;
}

pub trait AsObjectSerializer<'a> {
    type ObjectSerializer: ObjectSerializer;
    fn begin_object(&'a mut self) -> Self::ObjectSerializer;
}

pub trait AsArraySerializer<'a> {
    type ArraySerializer: ArraySerializer;
    fn begin_array(&'a mut self) -> Self::ArraySerializer;
}

pub trait ObjectSerializer {
    fn property<V: Serialize>(&mut self, name: &str, value: &V);
    fn end_object(self);
}

pub trait ArraySerializer {
    fn value<V: Serialize>(&mut self, value: &V);
    fn end_array(self);
}

pub trait Serialize {
    fn serialize<S: Serializer>(&self, serializer: &mut S);
}

impl Serialize for &str {
    fn serialize<S: Serializer>(&self, serializer: &mut S) {
        serializer.string(self)
    }
}

impl Serialize for String {
    fn serialize<S: Serializer>(&self, serializer: &mut S) {
        serializer.string(self)
    }
}

impl Serialize for i32 {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: &mut S) {
        serializer.i64(*self as i64)
    }
}

impl Serialize for i64 {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: &mut S) {
        serializer.i64(*self)
    }
}

impl Serialize for usize {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: &mut S) {
        serializer.i64(*self as i64)
    }
}

impl Serialize for f32 {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: &mut S) {
        serializer.f64(*self as f64)
    }
}

impl Serialize for f64 {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: &mut S) {
        serializer.f64(*self)
    }
}

impl Serialize for bool {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: &mut S) {
        serializer.bool(*self)
    }
}

impl<SERIALIZE: Serialize> Serialize for [SERIALIZE] {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: &mut S) {
        let mut array_serializer = serializer.begin_array();
        for value in self {
            array_serializer.value(value);
        }
        array_serializer.end_array();
    }
}

impl<SERIALIZE: Serialize, const SIZE: usize> Serialize for [SERIALIZE; SIZE] {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: &mut S) {
        let mut array_serializer = serializer.begin_array();
        for value in self {
            array_serializer.value(value);
        }
        array_serializer.end_array();
    }
}

impl<SERIALIZE: Serialize> Serialize for Vec<SERIALIZE> {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: &mut S) {
        let mut array_serializer = serializer.begin_array();
        for value in self {
            array_serializer.value(value);
        }
        array_serializer.end_array();
    }
}

impl<STRING: std::ops::Deref<Target = str>, V: Serialize> Serialize for HashMap<STRING, V> {
    fn serialize<S: Serializer>(&self, serializer: &mut S) {
        let mut object_serializer = serializer.begin_object();
        for (key, value) in self.into_iter() {
            object_serializer.property(key, value);
        }
        object_serializer.end_object();
    }
}

impl<SERIALIZE: Serialize> Serialize for Option<SERIALIZE> {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: &mut S) {
        if let Some(s) = self {
            s.serialize(serializer);
        } else {
            serializer.null()
        }
    }
}
