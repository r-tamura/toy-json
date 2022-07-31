# toy json

A toy json processor written in Rust

## Usage


### CLI

#### Install

[Download](https://github.com/r-tamura/toy-json/releases)

#### Example

```shell
$ cat <<EOT | ./target/release/tj
{"Hello, Wasm!": true, "list"
      : [1,
        2,

    null], "object": {
            "nonascii":
                 "日本語", "unicode": "\u30cf\u30ed\u30fc\u30ef\u30fc\u30eb\u30c9\u2600\ufe0f"

  }}
EOT

{
  "Hello, Wasm!": true,
  "list": [
    1,
    2,
    null
  ],
  "object": {
    "nonascii": "日本語",
    "unicode": "ハローワールド☀️"
  }
}
```

### Rust

```toml
[dependencies]
toy-json = { git = "https://github.com/r-tamura/toy-json.git" }
```

```shell
use toy_json::{ast, parse};

fn main() {
    let value = parse(r#"{"prop": 42}"#);

    if let Some(Ok(v)) = value {
        print!("{}", v.dump(&ast::DumpOptions::new(true, true, 2)));
    }
}
```

## Using from Python

[Using from Python](./examples/python/)

## Using on Web

[Using on Web](./examples/wasm/)

[Demo](https://rtam.xyz/toy-json)

## Development

## TODO

- [x] pretty print
- [x] colorize
- [x] wasm
- [x] unicode
- [x] ci
