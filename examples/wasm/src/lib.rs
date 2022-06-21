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
pub fn parse(s: &str) -> JsValue {
    utils::set_panic_hook();
    let opt = toy_json::parse(s);
    // let v = Object(vec![("Hello???".to_string(), Null)]);
    // s.to_string()
    match opt {
        Some(res) => JsValue::from_serde(&res.unwrap()).unwrap(),
        None => JsValue::NULL,
    }
}
