use wasm_bindgen::prelude::*;
use js_sys::Promise;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    fn getMouseCoordinates() -> Promise;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Use a função em Rust
#[wasm_bindgen]
pub async fn log_mouse_coordinates() {
    let coordinates_promise = getMouseCoordinates();
    let coordinates_jsvalue = wasm_bindgen_futures::JsFuture::from(coordinates_promise).await.unwrap();
    let coordinates: js_sys::Object = coordinates_jsvalue.into();
    let x = js_sys::Reflect::get(&coordinates, &"x".into()).unwrap().as_f64().unwrap();
    let y = js_sys::Reflect::get(&coordinates, &"y".into()).unwrap().as_f64().unwrap();
    log(&format!("Mouse coordinates: x = {}, y = {}", x, y));
}
