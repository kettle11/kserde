use crate::*;
use std::iter::Iterator;

pub struct JSONSerializer {
    s: String,
    indentation: u16,
}

impl JSONSerializer {
    fn indent(&mut self) {
        self.s.extend((0..self.indentation).map(|_| ' '))
    }
}

impl Serializer for JSONSerializer {
    fn new() -> Self {
        Self {
            s: String::new(),
            indentation: 0,
        }
    }

    type Result = String;

    fn f64(&mut self, n: f64) {
        self.s.push_str(&n.to_string())
    }

    fn i64(&mut self, n: i64) {
        self.s.push_str(&n.to_string())
    }

    fn bool(&mut self, b: bool) {
        if b {
            self.s.push_str("true")
        } else {
            self.s.push_str("false")
        }
    }

    fn string(&mut self, s: &str) {
        self.s.push('\"');
        self.s.push_str(s);
        self.s.push('\"');
    }

    fn null(&mut self) {
        self.s.push_str("null");
    }

    fn done(self) -> Self::Result {
        self.s
    }
}

pub struct JSONObjectSerializer<'a> {
    need_to_pop_comma: bool,
    serializer: &'a mut JSONSerializer,
}

impl<'a> AsObjectSerializer<'a> for JSONSerializer {
    type ObjectSerializer = JSONObjectSerializer<'a>;
    fn begin_object(&'a mut self) -> Self::ObjectSerializer {
        self.s.push_str("{\n");
        self.indentation += 4;
        JSONObjectSerializer {
            need_to_pop_comma: false,
            serializer: self,
        }
    }
}

impl<'a> ObjectSerializer for JSONObjectSerializer<'a> {
    fn property<V: Serialize>(&mut self, name: &str, value: &V) {
        let serializer = &mut self.serializer;
        serializer.s.push('\n');
        serializer.indent();
        name.serialize(*serializer);
        serializer.s.push_str(": ");
        value.serialize(*serializer);
        serializer.s.push_str(", ");
        self.need_to_pop_comma = true;
    }

    fn end_object(self) {
        let serializer = self.serializer;
        serializer.indentation -= 4;
        serializer.s.pop();

        if self.need_to_pop_comma {
            serializer.s.pop();
            serializer.s.push('\n');
            serializer.indent()
        }
        serializer.s.push_str("}");
    }
}

pub struct JSONArraySerializer<'a> {
    need_to_pop_comma: bool,
    serializer: &'a mut JSONSerializer,
}

impl<'a> AsArraySerializer<'a> for JSONSerializer {
    type ArraySerializer = JSONArraySerializer<'a>;
    fn begin_array(&'a mut self) -> Self::ArraySerializer {
        self.s.push_str("[");
        JSONArraySerializer {
            need_to_pop_comma: false,
            serializer: self,
        }
    }
}

impl<'a> ArraySerializer for JSONArraySerializer<'a> {
    fn value<V: Serialize>(&mut self, value: &V) {
        value.serialize(self.serializer);
        self.serializer.s.push(',');
        self.serializer.s.push(' ');
        self.need_to_pop_comma = true;
    }

    fn end_array(self) {
        if self.need_to_pop_comma {
            self.serializer.s.pop();
            self.serializer.s.pop();
        }
        self.serializer.s.push(']');
    }
}

pub fn to_json_string<V: Serialize>(value: &V) -> String {
    let mut serializer = JSONSerializer::new();
    value.serialize(&mut serializer);
    serializer.done()
}
