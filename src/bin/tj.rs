use core::fmt;
use std::{
    env, io,
    io::{Read, Write},
    process::exit,
};

use toy_json::{ast, parse};

fn ansi_color(s: impl fmt::Display, c: u32) -> String {
    format!("\x1b[{}m{}\x1b[0m", c, s)
}

fn red(s: impl fmt::Display) -> String {
    ansi_color(s, 31)
}

fn green(s: impl fmt::Display) -> String {
    ansi_color(s, 32)
}

fn yellow(s: impl fmt::Display) -> String {
    ansi_color(s, 33)
}

fn blue(s: impl fmt::Display) -> String {
    ansi_color(s, 34)
}

fn usage() {
    eprintln!("tj - commandline JSON processor\n");
    eprintln!(r#"usage: echo '{{"prop": "value"}}' | tj [options]"#)
}

fn print_value(
    value: &ast::Value,
    buf: &mut io::BufWriter<std::io::Stdout>,
    level: usize,
    options: &Options,
) -> Result<(), Box<dyn std::error::Error>> {
    use ast::Value::*;
    let indent = options.spaces * level;
    match value {
        Number(n) => write!(buf, "{}", n)?,
        Bool(b) => {
            if options.monochrome {
                write!(buf, "{}", b)?;
            } else {
                write!(buf, "{}", if *b { yellow(b) } else { blue(b) })?
            }
        }
        String(s) => write!(
            buf,
            r#""{}""#,
            if options.monochrome {
                s.to_string()
            } else {
                green(s)
            }
        )?,
        Null => write!(
            buf,
            "{}",
            if options.monochrome {
                "null".to_string()
            } else {
                red("null")
            }
        )?,
        Array(items) => {
            write!(buf, "[")?;
            if !options.compact {
                writeln!(buf)?;
            }
            let last_index = items.len() - 1;
            for (i, element) in items.iter().enumerate() {
                if !options.compact {
                    write!(buf, "{}", " ".repeat(indent + options.spaces))?;
                }
                print_value(element, buf, level + 1, options)?;
                if i != last_index {
                    write!(buf, ",")?;
                }
                if !options.compact {
                    writeln!(buf)?;
                }
            }
            if !options.compact {
                write!(buf, "{}", " ".repeat(indent))?;
            }
            write!(buf, "]")?;
        }
        Object(items) => {
            write!(buf, "{{")?;
            if !options.compact {
                writeln!(buf)?;
            }
            let last_index = items.len() - 1;
            let spaces = " ".repeat(indent + (options.spaces));
            for (i, (key, value)) in items.iter().enumerate() {
                if !options.compact {
                    write!(buf, "{}", spaces)?;
                }
                write!(
                    buf,
                    r#""{}""#,
                    if options.monochrome {
                        key.to_string()
                    } else {
                        blue(key)
                    }
                )?;
                write!(buf, ":")?;
                if !options.compact {
                    write!(buf, " ")?;
                }
                print_value(value, buf, level + 1, options)?;

                if i != last_index {
                    write!(buf, ",")?;
                }
                if !options.compact {
                    writeln!(buf)?;
                }
            }
            if !options.compact {
                write!(buf, "{}", " ".repeat(indent))?;
            }
            write!(buf, "}}")?;
        }
    };
    Ok(())
}

fn print_json(value: ast::Value, options: Options) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = io::BufWriter::new(std::io::stdout());
    // match value {}
    print_value(&value, &mut buf, 0, &options)?;
    writeln!(buf)?;
    Ok(())
}

struct Options {
    compact: bool,
    monochrome: bool,
    spaces: usize,
}

impl Options {
    fn new(compact: bool, monochrome: bool) -> Self {
        Options {
            compact,
            monochrome,
            spaces: 2,
        }
    }
}

fn main() {
    let (flags, _args): (Vec<String>, Vec<String>) = env::args()
        .into_iter()
        .skip(1)
        .partition(|arg| arg.starts_with('-'));

    let mut compact = false;
    let mut monochrome = false;
    flags.into_iter().for_each(|flag| match flag.as_str() {
        "-c" | "--compact" => {
            compact = true;
        }
        "-M" | "--monochrome" => {
            monochrome = true;
        }
        _ => {
            eprintln!("unsupported option: {}", flag)
        }
    });

    let options = Options::new(compact, monochrome);

    let mut buf = String::new();
    let res = io::stdin().read_to_string(&mut buf);
    if res.is_err() {
        eprintln!("{}", res.unwrap_err());
        exit(1);
    }

    let value = parse(&buf);
    match value {
        None => {
            usage();
            exit(1);
        }
        Some(Err(e)) => {
            usage();
            eprintln!("{}", e);
            exit(1);
        }
        Some(Ok(value)) => {
            print_json(value, options).unwrap();
        }
    };
}
