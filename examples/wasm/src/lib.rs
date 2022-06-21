mod utils;

use toy_json;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }

#[wasm_bindgen]
pub fn parse(s: &str) -> Result<JsValue, JsValue> {
    utils::set_panic_hook();
    let opt = toy_json::parse(s);
    match opt {
        Some(Ok(value)) => Ok(JsValue::from_serde(&value).unwrap()),
        Some(Err(e)) => Err(JsValue::from(format!("{}", e))),
        None => Err(JsValue::from("NoTokenFound")),
    }
}

#[wasm_bindgen]
pub fn format(s: &str, indent: usize) -> Result<String, JsValue> {
    utils::set_panic_hook();
    let opt = toy_json::parse(s);
    match opt {
        Some(Ok(value)) => {
            let dump_options = toy_json::ast::DumpOptions {
                color: false,
                pretty_print: true,
                indent,
            };
            Ok(value.dump(&dump_options))
        }
        Some(Err(e)) => Err(JsValue::from(format!("{}", e))),
        None => Err(JsValue::from("NoTokenFound")),
    }
}
