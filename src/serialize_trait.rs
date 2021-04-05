use std::collections::HashMap;

pub trait Serializer: for<'a> AsObjectSerializer<'a> + for<'a> AsArraySerializer<'a> {
    type Result;
    fn new() -> Self;
    fn string(&mut self, s: &str);
    fn bool(&mut self, b: bool);
    fn i64(&mut self, i: i64);
    fn f64(&mut self, n: f64);
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
    fn serialize<E: Serializer>(&self, serializer: &mut E);
}

impl Serialize for &str {
    fn serialize<E: Serializer>(&self, serializer: &mut E) {
        serializer.string(self)
    }
}

impl Serialize for i64 {
    #[inline]
    fn serialize<E: Serializer>(&self, serializer: &mut E) {
        serializer.i64(*self)
    }
}

impl Serialize for f64 {
    #[inline]
    fn serialize<E: Serializer>(&self, serializer: &mut E) {
        serializer.f64(*self)
    }
}

impl Serialize for bool {
    #[inline]
    fn serialize<E: Serializer>(&self, serializer: &mut E) {
        serializer.bool(*self)
    }
}

impl<S: Serialize> Serialize for [S] {
    #[inline]
    fn serialize<E: Serializer>(&self, serializer: &mut E) {
        let mut array_serializer = serializer.begin_array();
        for value in self.into_iter() {
            array_serializer.value(value);
        }
        array_serializer.end_array();
    }
}

impl<S: std::ops::Deref<Target = str>, V: Serialize> Serialize for HashMap<S, V> {
    fn serialize<E: Serializer>(&self, serializer: &mut E) {
        let mut object_serializer = serializer.begin_object();
        for (key, value) in self.into_iter() {
            object_serializer.property(key, value);
        }
        object_serializer.end_object();
    }
}
