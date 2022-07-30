#![allow(non_snake_case)]
// use assert_cmd::prelude::*;
use assert_cmd::Command;

#[test]
fn JSONが文字列だけのとき文字列が表示される() {
    let mut cmd = Command::cargo_bin("tj").unwrap();
    let assert = cmd.write_stdin(r#""string""#).args(&["-M"]).assert();
    assert.stdout("\"string\"\n");
}

#[test]
fn JSONがユニコードエスケープされた文字列だけのとき_ダブルクォーテーションで括られた文字列が表示される(
) {
    let mut cmd = Command::cargo_bin("tj").unwrap();
    let assert = cmd
        .write_stdin(r#""\u30CF\u30ED\u30FC\u30EF\u30FC\u30EB\u30C9\u2600\uFE0F""#)
        .args(&["-M"])
        .assert();
    assert.stdout("\"ハローワールド☀️\"\n");
}

const INPUT_JSON_OBJECT: &str = r#"{
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
  }]
}"#;

#[test]
fn フォーマットオプションが指定されないとき_インテント数に応じてJSONをフォーマットして表示する() {
    let mut cmd = Command::cargo_bin("tj").unwrap();
    let assert = cmd.args(&["-M"]).write_stdin(INPUT_JSON_OBJECT).assert();
    assert.stdout(
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
    );
}

#[test]
fn コンパクト表示のオプションがが指定されたとき_JSONオブジェクトがインラインで表示される() {
    let mut cmd = Command::cargo_bin("tj").unwrap();
    let assert = cmd
        .args(&["-c", "-M"])
        .write_stdin(INPUT_JSON_OBJECT)
        .assert();
    assert.stdout(
        r#"{"name":"Mr.X","age":30,"hasPet":true,"pets":[{"type":"cat","name":"John"},{"type":null,"name":"Cathy"}]}
"#,
    );
}
