use std::{
    env, io,
    io::{Read, Write},
    process::exit,
};

use toy_json::{ast, parse};

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
        Bool(b) => write!(buf, "{}", b)?,
        String(s) => write!(buf, r#""{}""#, s)?,
        Null => write!(buf, "null")?,
        Array(items) => {
            write!(buf, "[")?;
            if !options.compact {
                write!(buf, "\n")?;
            }
            let last_index = items.len() - 1;
            for (i, element) in items.iter().enumerate() {
                if !options.compact {
                    write!(buf, "{}", " ".repeat(indent + options.spaces))?;
                }
                print_value(element, buf, level+1, options)?;
                if i != last_index {
                    write!(buf, ",")?;
                }
                if !options.compact {
                    write!(buf, "\n")?;
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
                write!(buf, "\n")?;
            }
            let last_index = items.len() - 1;
            let spaces = " ".repeat(indent + (options.spaces));
            for (i, (key, value)) in items.iter().enumerate() {
                if !options.compact {
                    write!(buf, "{}", spaces)?;
                }
                write!(buf, r#""{}""#, key)?;
                write!(buf, ":")?;
                if !options.compact {
                    write!(buf, " ")?;
                }
                print_value(value, buf, level+1, options)?;

                if i != last_index {
                    write!(buf, ",")?;
                }
                if !options.compact {
                    write!(buf, "\n")?;
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
    write!(buf, "\n")?;
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
            compact,  monochrome, spaces: 2
        }
    }
}

fn main() {
    let (flags, args): (Vec<String>, Vec<String>) = env::args()
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
        Some(Err(e)) => eprintln!("{}", e),
        Some(Ok(value)) => {
            print_json(value, options).unwrap();
        }
    };
}
