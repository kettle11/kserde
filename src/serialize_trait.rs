use std::collections::HashMap;

pub trait Serializer {
    type Result;
    fn object<'a, S: Serialize + 'a, I: IntoIterator<Item = (&'a str, &'a S)>>(
        &mut self,
        members: I,
    );
    fn array<'a, S: Serialize + 'a, I: IntoIterator<Item = &'a S>>(&mut self, values: I);
    fn string(&mut self, s: &str);
    fn bool(&mut self, b: bool);
    fn i64(&mut self, i: i64);
    fn f64(&mut self, n: f64);
    fn done(self) -> Self::Result;
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

/*
impl<S: Serialize> Serialize for &S {
    #[inline]
    fn serialize<E: Serializer>(&self, serializer: &mut E) {}
}
*/

impl<S: Serialize> Serialize for [S] {
    #[inline]
    fn serialize<E: Serializer>(&self, serializer: &mut E) {
        serializer.array(self)
    }
}

impl<S: Serialize> Serialize for HashMap<&str, S> {
    #[inline]
    fn serialize<E: Serializer>(&self, serializer: &mut E) {
        serializer.object(self.iter().map(|(k, v)| (*k, v)))
    }
}

impl<S: Serialize> Serialize for HashMap<String, S> {
    #[inline]
    fn serialize<E: Serializer>(&self, serializer: &mut E) {
        serializer.object(self.iter().map(|(k, v)| (k.as_str(), v)))
    }
}
