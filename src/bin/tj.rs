use std::{env, io, io::Read, process::exit};

use toy_json::{ast, parse};

fn usage() {
    eprintln!("tj - commandline JSON processor\n");
    eprintln!(r#"usage: echo '{{"prop": "value"}}' | tj [options]"#)
}

struct Options {
    compact: bool,
    monochrome: bool,
    spaces: usize,
}

impl From<Options> for ast::DumpOptions {
    fn from(opt: Options) -> Self {
        ast::DumpOptions::new(!opt.monochrome, !opt.compact, opt.spaces)
    }
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
            let mut json_str = value.dump(&From::from(options));
            if !json_str.ends_with('\n') {
                json_str.push('\n');
            }
            print!("{}", json_str);
        }
    };
}
