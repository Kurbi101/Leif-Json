use crate::value::Value;
use ordered_float::OrderedFloat;
use std::collections::BTreeMap;
use std::str;

pub struct Parser<'a> {
    content: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(txt: &'a str) -> Self {
        Parser {
            content: txt,
            pos: 0,
        }
    }

    fn current_char(&mut self) -> Result<char, String> {
        let slice = &self.content[self.pos..];
        match slice.chars().next() {
            Some(c) => Ok(c),
            None => Err(self.parsing_error("End of file".to_string())),
        }
    }

    fn parsing_error(&self, msg: String) -> String {
        let line_start = self.content[..self.pos]
            .rfind('\n')
            .map(|i| i + 1)
            .unwrap_or(0);

        let line_end = self.content[self.pos..]
            .find('\n')
            .map(|i| self.pos + i)
            .unwrap_or(self.content.len());

        let line_content = &self.content[line_start..line_end];

        let line_number = self.content[..line_start]
            .chars()
            .filter(|&c| c == '\n')
            .count();

        let column = self.pos - line_start;

        format!(
            "Error at line {}, column {}: {}\n\n{}\n{:>width$}^",
            line_number,
            column,
            msg,
            line_content,
            "",
            width = column - 1
        )
    }

    fn read_char(&mut self) -> Result<char, String> {
        let c = self.current_char()?;
        self.pos += c.len_utf8();
        Ok(c)
    }

    fn skip_whitespace(&mut self) {
        if let Some(offset) = self.content[self.pos..].find(|c: char| !c.is_whitespace()) {
            self.pos += offset;
        } else {
            self.pos = self.content.len();
        }
    }

    fn expect(&mut self, expected: char) -> Result<(), String> {
        let c = self.read_char()?;
        if c == expected {
            Ok(())
        } else {
            Err(self.parsing_error(format!("Expected: '{}', but found : '{}'", expected, c)))
        }
    }

    fn parse_word(&mut self) -> Result<Value, String> {
        let buffer: String = self.content[self.pos..]
            .chars()
            .take_while(|c| c.is_alphabetic())
            .collect();

        self.pos += buffer.len();

        match buffer.as_str() {
            "true" => Ok(Value::True),
            "false" => Ok(Value::False),
            "null" => Ok(Value::Null),
            _ => Err(self.parsing_error(format!("Found invalid identifier: {}", buffer))),
        }
    }

    fn parse_number(&mut self) -> Result<Value, String> {
        let number: String = self.content[self.pos..]
            .chars()
            .take_while(|c| *c == '-' || *c == 'e' || *c == 'E' || *c == '.' || c.is_numeric())
            .collect();

        self.pos += number.len();

        match number.parse::<i64>() {
            Ok(num) => Ok(Value::Int(num)),
            Err(_) => match number.parse::<f64>() {
                Ok(num) => Ok(Value::Float(OrderedFloat(num))),
                Err(e) => Err(self.parsing_error(e.to_string())),
            },
        }
    }

    fn parse_array(&mut self) -> Result<Value, String> {
        self.expect('[')?;
        self.skip_whitespace();
        let mut arr: Vec<Value> = Vec::new();
        while self.current_char()? != ']' {
            arr.push(self.parse_value()?);
            self.skip_whitespace();
            if self.current_char()? != ']' {
                self.expect(',')?;
                self.skip_whitespace();
            }
        }
        self.expect(']')?;
        Ok(Value::Array(arr))
    }

    fn parse_string(&mut self) -> Result<Value, String> {
        self.expect('"')?;
        let buffer: String = self.content[self.pos..]
            .chars()
            .take_while(|c| *c != '"')
            .collect();

        self.pos += buffer.len();

        self.expect('"')?;
        Ok(Value::String(buffer))
    }

    fn parse_object(&mut self) -> Result<Value, String> {
        self.expect('{')?;
        self.skip_whitespace();
        let mut table: BTreeMap<Value, Value> = BTreeMap::new();
        while self.current_char()? != '}' {
            let name = self.parse_string()?;
            self.skip_whitespace();
            self.expect(':')?;
            self.skip_whitespace();
            let val = self.parse_value()?;
            self.skip_whitespace();
            if self.current_char()? != '}' {
                self.expect(',')?;
                self.skip_whitespace();
            }
            table.insert(name, val);
        }
        self.expect('}')?;
        Ok(Value::Object(table))
    }

    pub fn parse_value(&mut self) -> Result<Value, String> {
        self.skip_whitespace();
        match self.current_char()? {
            '{' => self.parse_object(),
            '[' => self.parse_array(),
            'a'..='z' => self.parse_word(),
            '"' => self.parse_string(),
            '-' | '0'..='9' => self.parse_number(),
            _ => Err(format!("Couldn't parse char: {}", self.current_char()?)),
        }
    }
}
