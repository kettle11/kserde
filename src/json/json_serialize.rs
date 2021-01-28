use crate::*;
use std::borrow::Borrow;
use std::iter::Iterator;

pub struct JSONSerializer {
    s: String,
    indentation: u16,
}

impl JSONSerializer {
    pub fn new() -> Self {
        Self {
            s: String::new(),
            indentation: 0,
        }
    }

    fn indent(&mut self) {
        self.s.extend((0..self.indentation).map(|_| ' '))
    }
}

impl Serializer for JSONSerializer {
    type Result = String;
    fn map<'a, S: Serialize + 'a, I: IntoIterator<Item = (&'a str, &'a S)>>(&mut self, members: I) {
        self.s.push_str("{\n");
        self.indentation += 4;
        let mut more_than_one_item = false;
        for (key, value) in members {
            self.indent();
            key.serialize(self);
            self.s.push_str(": ");
            value.serialize(self);
            self.s.push(',');
            self.s.push('\n');
            more_than_one_item = true;
        }
        self.indentation -= 4;

        self.s.pop();

        if more_than_one_item {
            self.s.pop(); // Pop extra comma and newline. This is incorrect for empty objects.
            self.s.push('\n');
            self.indent()
        }
        self.s.push_str("}");
    }

    fn array<'a, S: Serialize + 'a, I: IntoIterator<Item = &'a S>>(&mut self, values: I) {
        self.s.push('[');
        let mut more_than_one_value = false;
        for value in values {
            value.serialize(self);
            self.s.push_str(", ");
            more_than_one_value = true;
        }
        if more_than_one_value {
            self.s.pop(); // Pop extra comma and space This is incorrect for empty objects.
            self.s.pop();
        }
        self.s.push(']');
    }

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

    fn done(self) -> Self::Result {
        self.s
    }
}

impl<'a> Serialize for Value<'a> {
    fn serialize<E: Serializer>(&self, serializer: &mut E) {
        match self {
            Value::Object(values) => serializer.map(values.iter().map(|(k, v)| (k.borrow(), v))),
            Value::Array(values) => {
                serializer.array(values);
            }
            Value::String(s0) => {
                serializer.string(s0);
            }
            // Potentially unnecessary heap allocation?
            Value::Number(n) => serializer.f64(*n),
            Value::Boolean(b) => serializer.bool(*b),

            // This is incorrect as it should not have quotes in JSON.
            Value::Null => serializer.string("null"),
        }
    }
}

pub fn to_json_string<V: Serialize>(value: &V) -> String {
    let mut serializer = JSONSerializer::new();
    value.serialize(&mut serializer);
    serializer.done()
}
