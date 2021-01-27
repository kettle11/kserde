use crate::Value;
use std::borrow::Cow;
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::CharIndices;

const RECURSIVE_LIMIT: usize = 1024;

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
                    let next = self.iter.next()?;
                    match next.1 {
                        '\"' => string.to_mut().push('"'),
                        '/' => string.to_mut().push('/'),
                        '\\' => string.to_mut().push('\\'),
                        'n' => string.to_mut().push('\n'),
                        'b' => string.to_mut().push('\x08'),
                        'f' => string.to_mut().push('\x0C'),
                        'r' => string.to_mut().push('\r'),
                        't' => string.to_mut().push('\t'),
                        'u' => {
                            let slice = self.source.get(next.0 + 1..next.0 + 5)?;
                            let u = u32::from_str_radix(slice, 16).ok()?;
                            for _ in 0..4 {
                                self.iter.next();
                            }

                            let c = match u {
                                0xD800..=0xDBFF => {
                                    // This is a non-Basic Multilingual Plane (BMP) character
                                    // so it's encoded as two code points.

                                    // Skip the '\u'
                                    self.iter.next()?;
                                    let (start, _) = self.iter.next()?;

                                    let slice = self.source.get(start + 1..start + 5)?;
                                    let u1 = u32::from_str_radix(slice, 16).ok()?;
                                    if u1 < 0xDC00 || u1 > 0xDFFF {
                                        return None;
                                    }
                                    let n = (u32::from(u - 0xD800) << 10 | u32::from(u1 - 0xDC00))
                                        + 0x1_0000;

                                    for _ in 0..4 {
                                        self.iter.next();
                                    }
                                    std::char::from_u32(n)?
                                }
                                _ => std::char::from_u32(u)?,
                            };

                            string.to_mut().push(c);
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

        match self.iter.peek()?.1 {
            '0' => {
                self.iter.next();
            }
            c if c.is_ascii_digit() => loop {
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
            _ => return None,
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
                self.iter.next();
                let sign = match self.iter.peek()?.1 {
                    '-' => {
                        self.iter.next();
                        -1.
                    }
                    '+' => {
                        self.iter.next();
                        1.
                    }
                    _ => 1.,
                };

                let mut exponent = 0.0;
                match self.iter.peek()?.1 {
                    // Are leading 0s for the exponents really something that happens?
                    '0' => {
                        self.iter.next();
                    }
                    c if c.is_ascii_digit() => loop {
                        if let Some((_, c)) = self.iter.peek().cloned() {
                            if let Some(digit) = c.to_digit(10) {
                                exponent *= 10.;
                                exponent += digit as f64;
                                self.iter.next();
                                continue;
                            }
                        }
                        break;
                    },
                    _ => return None,
                }

                number = number.powf(exponent * sign);
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

        Some(match self.iter.peek()?.1 {
            '{' => {
                self.iter.next();
                Value::Object(self.parse_object()?)
            }
            '[' => {
                self.iter.next();
                Value::Array(self.parse_array()?)
            }
            '"' => {
                Value::String(self.parse_string()?)
                // Parse String
            }
            't' => {
                // Parse true
                // For now just assume all the characters are correct
                for _ in 0..4 {
                    self.iter.next()?;
                }
                Value::Boolean(true)
            }
            'f' => {
                // Parse false
                // For now just assume all the characters are correct
                for _ in 0..5 {
                    self.iter.next()?;
                }
                Value::Boolean(true)
            }
            'n' => {
                // Parse null
                // For now just assume all the characters are correct
                for _ in 0..4 {
                    self.iter.next()?;
                }
                Value::Boolean(true)
            }
            '-' => {
                // Parse negative number
                Value::Number(self.parse_number()?)
            }
            c if c.is_ascii_digit() => Value::Number(self.parse_number()?),
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
