# toy json

A toy json processor witten in Rust

## Usage


### CLI

```shell
cat <<EOT | ./target/release/tj
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

## Development

## TODO

- [x] pretty print
- [x] colorize
- [x] wasm
- [x] unicode
- [x] ci


