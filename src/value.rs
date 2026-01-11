use ordered_float::OrderedFloat;
use std::collections::BTreeMap;
use std::fmt;

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

struct ValuePrinter<'a> {
    value: &'a Value,
    num_indents: usize,
}

impl<'a> ValuePrinter<'a> {
    pub fn new(val: &'a Value, num_idents: usize) -> Self {
        ValuePrinter {
            value: val,
            num_indents: num_idents,
        }
    }

    pub fn write_indentation(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for _ in 0..self.num_indents {
            f.write_str("  ")?
        }
        Ok(())
    }
}

impl<'a> fmt::Display for ValuePrinter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_indentation(f)?;
        match self.value {
            Value::Null => write!(f, "null"),
            Value::True => write!(f, "true"),
            Value::False => write!(f, "false"),
            Value::Int(num) => write!(f, "{}", num),
            Value::Float(num) => write!(f, "{}", num),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Array(vec) => match vec.is_empty() {
                true => write!(f, "[]"),
                false => {
                    let mut iter = vec.iter().peekable();
                    f.write_str("[\n")?;
                    while let Some(val) = iter.next() {
                        f.write_fmt(format_args!(
                            "{}",
                            ValuePrinter::new(val, self.num_indents + 1)
                        ))?;
                        if !iter.peek().is_none() {
                            f.write_str(",")?;
                        }
                        f.write_str("\n")?;
                    }
                    self.write_indentation(f)?;
                    f.write_str("]")
                }
            },
            Value::Object(map) => match map.is_empty() {
                true => write!(f, "{{}}"),
                false => {
                    f.write_str("{\n")?;
                    let mut iter = map.iter().peekable();
                    while let Some((key, val)) = iter.next() {
                        let _ = f.write_fmt(format_args!(
                            "{}: {}",
                            ValuePrinter::new(key, self.num_indents + 1),
                            ValuePrinter::new(val, 0)
                        ));
                        if !iter.peek().is_none() {
                            f.write_str(",")?;
                        }
                        f.write_str("\n")?;
                    }
                    self.write_indentation(f)?;
                    f.write_str("}")
                }
            },
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", ValuePrinter::new(self, 0))
    }
}
