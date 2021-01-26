use std::borrow::Cow;
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::CharIndices;

const RECURSIVE_LIMIT: usize = 1000;

#[derive(Debug, Clone)]
pub enum Value<'a> {
    String(Cow<'a, str>),
    Number(f64),
    Object(HashMap<Cow<'a, str>, Value<'a>>),
    Array(Vec<Value<'a>>),
    Boolean(bool),
    Null,
}

impl<'a> Value<'a> {
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Value::String(a) => Some(&a),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match *self {
            Value::Number(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&HashMap<Cow<'a, str>, Value<'a>>> {
        match self {
            Value::Object(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<Value<'a>>> {
        match self {
            Value::Array(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        match *self {
            Value::Boolean(b) => Some(b),
            _ => None,
        }
    }
}

struct Parser<'a> {
    recursion_depth: usize,
    source: &'a str,
    iter: Peekable<CharIndices<'a>>,
}

impl<'a> Parser<'a> {
    pub fn skip_whitespace(&mut self) {
        while self.iter.peek().map_or(false, |(_, c)| c.is_whitespace()) {
            self.iter.next();
        }
    }

    pub fn parse_string(&mut self) -> Option<Cow<'a, str>> {
        let start_index = match self.iter.next() {
            Some((i, '"')) => i,
            _ => return None,
        };

        let mut string = Cow::from("");
        let mut owned = false;

        loop {
            match self.iter.next() {
                Some((_, '"')) => break,
                Some((_, '\\')) => {
                    owned = true;
                    match self.iter.next() {
                        Some((_, '\"')) => string.to_mut().push('"'),
                        Some((_, '/')) => string.to_mut().push('/'),
                        Some((_, '\\')) => string.to_mut().push('\\'),
                        Some((_, 'n')) => string.to_mut().push('\n'),
                        Some((_, 'b')) => string.to_mut().push('\x08'),
                        Some((_, 'f')) => string.to_mut().push('\x0C'),
                        Some((_, 'r')) => string.to_mut().push('\r'),
                        Some((_, 't')) => string.to_mut().push('\t'),
                        Some((_, 'u')) => {
                            /*
                            let b0 = self.iter.next()?.1;
                            let b1 = self.iter.next()?.1;
                            let b2 = self.iter.next()?.1;
                            let b3 = self.iter.next()?.1;
                            */

                            // Not yet implemented
                            return None;
                        }
                        _ => return None,
                    }
                }
                None => return None,
                Some((i, c)) => {
                    if owned {
                        string.to_mut().push(c)
                    } else {
                        string = Cow::from(&self.source[start_index + 1..i + c.len_utf8()])
                    }
                }
            }
        }

        Some(string)
    }

    pub fn parse_object(&mut self) -> Option<HashMap<Cow<'a, str>, Value<'a>>> {
        // '{' already parsed
        let mut string_to_value = HashMap::new();
        loop {
            self.skip_whitespace();
            match self.iter.peek() {
                Some((_, ',')) => {
                    self.iter.next();
                }
                Some((_, '}')) => {
                    self.iter.next();
                    break;
                }
                None => return None,
                _ => {}
            }

            self.skip_whitespace();
            let string = self.parse_string()?;

            self.skip_whitespace();
            match self.iter.next() {
                Some((_, ':')) => {}
                _ => return None,
            };

            self.skip_whitespace();

            let value = self.parse_value()?;
            string_to_value.insert(string, value);
        }
        Some(string_to_value)
    }

    pub fn parse_array(&mut self) -> Option<Vec<Value<'a>>> {
        let mut values = Vec::new();
        loop {
            self.skip_whitespace();
            match self.iter.peek() {
                Some((_, ',')) => {
                    self.iter.next();
                }
                Some((_, ']')) => {
                    self.iter.next();
                    break;
                }
                None => {
                    return None;
                }
                _ => {}
            }
            self.skip_whitespace();

            values.push(self.parse_value()?);
        }
        Some(values)
    }

    pub fn parse_number(&mut self) -> Option<f64> {
        let is_negative = match self.iter.peek() {
            Some((_, '-')) => {
                self.iter.next();
                true
            }
            _ => false,
        };

        let mut number = 0.0;

        match self.iter.peek() {
            Some((_, '0')) => {
                self.iter.next();
            }
            Some((_, c)) if c.is_ascii_digit() => loop {
                if let Some((_, c)) = self.iter.peek().cloned() {
                    if let Some(digit) = c.to_digit(10) {
                        number *= 10.;
                        number += digit as f64;
                        self.iter.next();
                        continue;
                    }
                }
                break;
            },
            _ => None?,
        }

        let mut position = 10.0;
        // Parse fraction
        match self.iter.peek() {
            Some((_, '.')) => {
                self.iter.next();
                // Parse fraction
                loop {
                    if let Some((_, c)) = self.iter.peek().cloned() {
                        if let Some(digit) = c.to_digit(10) {
                            number += digit as f64 / position;
                            position *= 10.0;
                            self.iter.next();
                            continue;
                        }
                    }
                    break;
                }
            }
            _ => {}
        }

        // Parse exponent
        match self.iter.peek() {
            Some((_, 'e')) | Some((_, 'E')) => {
                // Unimplemented
                return None;
                // Parse fraction
                /*
                self.iter.next();

                match self.iter.next() {
                    Some((_, '-')) => {}
                    Some((_, '+')) => {}
                    _ => return None,
                }

                while self.iter.next().map_or(false, |(_, c)| c.is_ascii_digit()) {}
                */
            }
            _ => {}
        }

        if is_negative {
            number *= -1.0;
        }
        Some(number)
    }

    pub fn parse_value(&mut self) -> Option<Value<'a>> {
        self.recursion_depth += 1;
        if self.recursion_depth > RECURSIVE_LIMIT {
            return None;
        }
        self.skip_whitespace();

        Some(match self.iter.peek() {
            Some((_, '{')) => {
                self.iter.next();
                Value::Object(self.parse_object()?)
            }
            Some((_, '[')) => {
                self.iter.next();
                Value::Array(self.parse_array()?)
            }
            Some((_, '"')) => {
                Value::String(self.parse_string()?)
                // Parse String
            }
            Some((_, 't')) => {
                // Parse true
                // For now just assume all the characters are correct
                for _ in 0..4 {
                    self.iter.next()?;
                }
                Value::Boolean(true)
            }
            Some((_, 'f')) => {
                // Parse false
                // For now just assume all the characters are correct
                for _ in 0..5 {
                    self.iter.next()?;
                }
                Value::Boolean(true)
            }

            Some((_, 'n')) => {
                // Parse null
                // For now just assume all the characters are correct
                for _ in 0..4 {
                    self.iter.next()?;
                }
                Value::Boolean(true)
            }
            Some((_, '-')) => {
                // Parse negative number
                Value::Number(self.parse_number()?)
            }
            Some((_, c)) if c.is_ascii_digit() => Value::Number(self.parse_number()?),
            _ => return None,
        })
    }
}
pub fn from_str<'a>(source: &'a str) -> Option<Value<'a>> {
    let mut parser = Parser {
        recursion_depth: 0,
        source,
        iter: source.char_indices().peekable(),
    };
    parser.parse_value()
}
