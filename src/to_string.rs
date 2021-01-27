use crate::Value;

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
            s.pop(); // Pop extra comma and newline
            s.pop();
            s.push('\n');
            indent(s, indentation);
            s.push_str("}");
        }
        Value::Array(values) => {
            s.push('[');
            for value in values {
                encode_value(s, indentation, value);
                s.push_str(", ");
            }
            s.pop(); // Pop extra comma and space
            s.pop();
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

pub fn to_string(value: &Value) -> String {
    let mut s = String::new();
    encode_value(&mut s, 0, value);
    s
}
