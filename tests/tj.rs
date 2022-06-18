#![allow(non_snake_case)]
// use assert_cmd::prelude::*;
use assert_cmd::Command;

#[test]
fn JSONが文字列だけのとき文字列が表示される() {
    let mut cmd = Command::cargo_bin("tj").unwrap();
    let assert = cmd.write_stdin(r#""string""#).args(&["-M"]).assert();
    assert.stdout("\"string\"\n");
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
