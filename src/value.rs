use ordered_float::OrderedFloat;
use std::collections::BTreeMap;

#[derive(PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum Value {
    Object(BTreeMap<Value, Value>),
    Array(Vec<Value>),
    String(String),
    Int(i64),
    Float(OrderedFloat<f64>),
    True,
    False,
    Null,
}

impl Value {
    fn tabbed_stringify(&self, num_tabs: usize) -> String {
        format!(
            "{}{}",
            "  ".repeat(num_tabs),
            match self {
                Value::Null => "null".to_string(),
                Value::False => "false".to_string(),
                Value::True => "true".to_string(),
                Value::Float(f) => f.to_string(),
                Value::Int(i) => i.to_string(),
                Value::String(s) => format!("\"{}\"", s),
                Value::Array(vec) => {
                    match vec.is_empty() {
                        true => "[]".to_string(),
                        false => {
                            let mut buffer = "[\n".to_string();
                            let mut iter = vec.iter().peekable();
                            while let Some(val) = iter.next() {
                                buffer += &val.tabbed_stringify(num_tabs + 1);
                                if !iter.peek().is_none() {
                                    buffer.push(',');
                                }
                                buffer.push('\n');
                            }
                            buffer += &"  ".repeat(num_tabs);
                            buffer += "]";
                            buffer
                        }
                    }
                }
                Value::Object(map) => {
                    match map.is_empty() {
                        true => "{}".to_string(),
                        false => {
                            let mut buffer = "{\n".to_string();
                            let mut iter = map.iter().peekable();
                            while let Some((key, val)) = iter.next() {
                                buffer += &key.tabbed_stringify(num_tabs + 1);
                                buffer += ": ";
                                buffer += &val.stringify();
                                if !iter.peek().is_none() {
                                    buffer.push(',')
                                }
                                buffer.push('\n');
                            }
                            buffer += &"  ".repeat(num_tabs);
                            buffer += "}";
                            buffer
                        }
                    }
                }
            }
        )
    }

    pub fn stringify(&self) -> String {
        self.tabbed_stringify(0)
    }
}
