use crate::*;
use std::iter::Iterator;
fn indent(s: &mut String, amount: u16) {
    s.extend((0..amount).map(|_| ' '))
}

fn encode_value(s: &mut String, indentation: u16, value: &Value) {
    match value {
        Value::Object(values) => {
            s.push_str("{\n");
            let indentation_inner = indentation + 4;
            for (key, value) in values {
                indent(s, indentation_inner);
                s.push('\"');
                s.push_str(key);
                s.push('\"');
                s.push_str(": ");
                encode_value(s, indentation_inner, value);
                s.push(',');
                s.push('\n');
            }
            s.pop();

            if values.len() > 0 {
                s.pop(); // Pop extra comma and newline. This is incorrect for empty objects.
                s.push('\n');
                indent(s, indentation);
            }
            s.push_str("}");
        }
        Value::Array(values) => {
            s.push('[');
            for value in values {
                encode_value(s, indentation, value);
                s.push_str(", ");
            }
            if values.len() > 0 {
                s.pop(); // Pop extra comma and space This is incorrect for empty objects.
                s.pop();
            }
            s.push(']');
        }
        Value::String(s0) => {
            s.push('\"');
            s.push_str(s0);
            s.push('\"');
        }
        // Potentially unnecessary heap allocation?
        Value::Number(n) => s.push_str(&n.to_string()),
        Value::Boolean(true) => s.push_str("true"),
        Value::Boolean(false) => s.push_str("false"),
        Value::Null => s.push_str("null"),
    }
}

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
    fn object<K: Serialize, S: Serialize, I: IntoIterator<Item = (S, S)>>(&mut self, members: I) {
        self.s.push_str("{\n");
        self.indentation += 4;
        let mut more_than_one_item = false;
        for (key, value) in members {
            self.indent();
            self.s.push('\"');
            key.serialize(self);
            self.s.push('\"');
            self.s.push_str(": ");
            value.serialize(self);
            self.s.push(',');
            self.s.push('\n');
            more_than_one_item = true;
        }
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
/*
impl<'a> Serialize for Value<'a> {
    fn serialize<E: Serializer>(&self, serializer: &mut E) {
        match self {
            Value::Object(values) => {
                s.push_str("{\n");
                let indentation_inner = indentation + 4;
                for (key, value) in values {
                    indent(s, indentation_inner);
                    s.push('\"');
                    s.push_str(key);
                    s.push('\"');
                    s.push_str(": ");
                    encode_value(s, indentation_inner, value);
                    s.push(',');
                    s.push('\n');
                }
                s.pop();

                if values.len() > 0 {
                    s.pop(); // Pop extra comma and newline. This is incorrect for empty objects.
                    s.push('\n');
                    indent(s, indentation);
                }
                s.push_str("}");
            }
            Value::Array(values) => {
                s.push('[');
                for value in values {
                    encode_value(s, indentation, value);
                    s.push_str(", ");
                }
                if values.len() > 0 {
                    s.pop(); // Pop extra comma and space This is incorrect for empty objects.
                    s.pop();
                }
                s.push(']');
            }
            Value::String(s0) => {
                s.push('\"');
                s.push_str(s0);
                s.push('\"');
            }
            // Potentially unnecessary heap allocation?
            Value::Number(n) => s.push_str(&n.to_string()),
            Value::Boolean(true) => s.push_str("true"),
            Value::Boolean(false) => s.push_str("false"),
            Value::Null => s.push_str("null"),
        }
    }
}
*/

pub fn to_string(value: &Value) -> String {
    let mut s = String::new();
    encode_value(&mut s, 0, value);
    s
}
