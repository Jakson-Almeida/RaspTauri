use sycamore::prelude::*;
use sycamore::futures::spawn_local_scoped;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    fn getMouseCoordinates() -> js_sys::Promise;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[component]
pub fn MouseTracker<G: Html>(cx: Scope) -> View<G> {
    let mouse_x = create_signal(cx, 0);
    let mouse_y = create_signal(cx, 0);

    log("Component initialized");

    // Defina os valores fixos aqui para teste
    mouse_x.set(50);
    mouse_y.set(50);
    log(&format!("Initial set: Mouse X: {}, Mouse Y: {}", mouse_x.get(), mouse_y.get()));

    let update_coordinates = {
        let mouse_x = mouse_x;
        let mouse_y = mouse_y;
        move || {
            log("Updating coordinates");
            let promise = getMouseCoordinates();
            log("Promise obtained");

            let future = wasm_bindgen_futures::JsFuture::from(promise);
            log("Future created");

            spawn_local_scoped(cx, async move {
                log("spawn_local_scoped()");
                if let Ok(coords) = future.await {
                    log("Promise resolved");
                    let coords_obj = js_sys::Object::unchecked_from_js(coords);
                    let x = js_sys::Reflect::get(&coords_obj, &JsValue::from_str("x")).unwrap().as_f64().unwrap() as i32;
                    let y = js_sys::Reflect::get(&coords_obj, &JsValue::from_str("y")).unwrap().as_f64().unwrap() as i32;
                    mouse_x.set(x);
                    mouse_y.set(y);
                    log(&format!("Mouse X: {}, Mouse Y: {}", x, y));
                } else {
                    log("Promise rejected");
                }
            });
        }
    };

    spawn_local_scoped(cx, async move {
        log("Starting loop");
        loop {
            log("Before update_coordinates()");
            update_coordinates();
            log("After update_coordinates()");

            // Adiciona um pequeno atraso entre as iterações do loop
            wasm_bindgen_futures::JsFuture::from(js_sys::Promise::resolve(&JsValue::undefined())).await.unwrap();
        }
    });

    view! { cx,
        div {
            h2 { "Mouse Tracker" }
            p { "Mouse X: " (mouse_x.get()) }
            p { "Mouse Y: " (mouse_y.get()) }
        }
    }
}
