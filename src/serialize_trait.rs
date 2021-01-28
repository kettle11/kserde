use std::collections::HashMap;

pub trait Serializer {
    type Result;
    fn object<K: Serialize, S: Serialize, I: IntoIterator<Item = (S, S)>>(&mut self, members: I);
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

impl Serialize for str {
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
        serializer.array(self)
    }
}

/*
impl<'a, K: Serialize + 'a, S: Serialize + 'a> Serialize for HashMap<K, S>
where
    HashMap<K, S>: IntoIterator<Item = (&'a K, &'a S)>,
{
    #[inline]
    fn serialize<E: Serializer>(&self, serializer: &mut E) {
        serializer.object(self.iter())
    }
}
*/
