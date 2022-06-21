use serde::{Deserialize, Serialize};

use crate::ansi_color;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Object(Vec<(String, Value)>),
    Array(Vec<Value>),
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

pub struct DumpOptions {
    pub color: bool,
    pub pretty_print: bool,
    pub indent: usize,
}

impl DumpOptions {
    pub fn new(color: bool, pretty_print: bool, indent: usize) -> Self {
        Self {
            color,
            pretty_print,
            indent,
        }
    }
}

impl Default for DumpOptions {
    fn default() -> Self {
        DumpOptions {
            color: false,
            pretty_print: false,
            indent: 2,
        }
    }
}

impl ToString for Value {
    fn to_string(&self) -> String {
        self.dump(&DumpOptions::default())
    }
}

impl Value {
    pub fn dump(&self, options: &DumpOptions) -> String {
        let mut s = Self::dump_value(self, 0, options);
        if options.pretty_print {
            s.push('\n');
        }
        s
    }

    pub fn dump_value(value: &Value, level: usize, options: &DumpOptions) -> String {
        use Value::*;
        let indent = options.indent * level;
        match value {
            Number(n) => n.to_string(),
            Bool(b) => {
                if !options.color {
                    b.to_string()
                } else if *b {
                    ansi_color::yellow(b)
                } else {
                    ansi_color::blue(b)
                }
            }
            String(s) => {
                format!(
                    r#""{}""#,
                    if !options.color {
                        s.to_string()
                    } else {
                        ansi_color::green(s)
                    }
                )
            }
            Null => {
                if !options.color {
                    "null".to_string()
                } else {
                    ansi_color::red("null")
                }
            }
            Array(items) => {
                let mut s = std::string::String::new();
                s.push('[');
                if options.pretty_print {
                    s.push('\n');
                }
                let last_index = items.len() - 1;
                for (i, element) in items.iter().enumerate() {
                    if options.pretty_print {
                        s.push_str(&" ".repeat(indent + options.indent));
                    }
                    s.push_str(&Self::dump_value(element, level + 1, options));
                    if i != last_index {
                        s.push(',');
                    }
                    if options.pretty_print {
                        s.push('\n');
                    }
                }
                if options.pretty_print {
                    s.push_str(&" ".repeat(indent));
                }
                s.push(']');
                s
            }
            Object(items) => {
                let mut s = std::string::String::new();
                s.push('{');
                if options.pretty_print {
                    s.push('\n');
                }
                let last_index = items.len() - 1;
                let spaces = " ".repeat(indent + (options.indent));
                for (i, (key, value)) in items.iter().enumerate() {
                    if options.pretty_print {
                        s.push_str(&spaces);
                    }
                    let key = format!(
                        r#""{}""#,
                        if !options.color {
                            key.to_string()
                        } else {
                            ansi_color::blue(key)
                        }
                    );
                    s.push_str(&key);
                    s.push(':');
                    if options.pretty_print {
                        s.push(' ');
                    }
                    s.push_str(&Self::dump_value(value, level + 1, options));

                    if i != last_index {
                        s.push(',');
                    }
                    if options.pretty_print {
                        s.push('\n');
                    }
                }
                if options.pretty_print {
                    s.push_str(&" ".repeat(indent));
                }
                s.push('}');
                s
            }
        }
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::Value::*;
    use super::*;

    #[test]
    fn JSONが文字列だけのとき_その文字列が出力される() {
        let value = String("Hello, toyjson!".to_string());
        let actual = value.to_string();
        assert_eq!(actual, r#""Hello, toyjson!""#.to_string());
    }

    #[test]
    fn to_stringでは出力文字列をフォーマットしない() {
        let BASIC_OBJECT: Value = Object(vec![
            ("name".to_string(), String("Mr.X".to_string())),
            ("age".to_string(), Number(30f64)),
            ("hasPet".to_string(), Bool(true)),
            (
                "pets".to_string(),
                Array(vec![
                    Object(vec![
                        ("type".to_string(), String("cat".to_string())),
                        ("name".to_string(), String("John".to_string())),
                    ]),
                    Object(vec![
                        ("type".to_string(), Null),
                        ("name".to_string(), String("Cathy".to_string())),
                    ]),
                ]),
            ),
        ]);
        let value = BASIC_OBJECT;
        let actual = value.to_string();
        assert_eq!(
            actual,
            r#"{"name":"Mr.X","age":30,"hasPet":true,"pets":[{"type":"cat","name":"John"},{"type":null,"name":"Cathy"}]}"#,
        );
    }

    #[test]
    fn format() {
        assert_eq!(format!(r#""{}""#, "???"), r#""???""#);
    }

    #[test]
    fn フォーマットオプションが指定されたとき_フォーマットしたJSON文字列を出力する() {
        let BASIC_OBJECT: Value = Object(vec![
            ("name".to_string(), String("Mr.X".to_string())),
            ("age".to_string(), Number(30f64)),
            ("hasPet".to_string(), Bool(true)),
            (
                "pets".to_string(),
                Array(vec![
                    Object(vec![
                        ("type".to_string(), String("cat".to_string())),
                        ("name".to_string(), String("John".to_string())),
                    ]),
                    Object(vec![
                        ("type".to_string(), Null),
                        ("name".to_string(), String("Cathy".to_string())),
                    ]),
                ]),
            ),
        ]);
        let value = BASIC_OBJECT;
        let actual = value.dump(&DumpOptions {
            pretty_print: true,
            ..DumpOptions::default()
        });
        assert_eq!(
            actual,
            r#"{
  "name": "Mr.X",
  "age": 30,
  "hasPet": true,
  "pets": [
    {
      "type": "cat",
      "name": "John"
    },
    {
      "type": null,
      "name": "Cathy"
    }
  ]
}
"#,
        )
    }
}
