#[allow(unused_imports)] // This JsValue is needed for some reason
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[wasm_bindgen(module = "/js/utils.js")]
extern "C" {
    pub fn getMouseCoordinates() -> js_sys::Promise;
}
