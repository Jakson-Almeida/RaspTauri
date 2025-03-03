use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use sycamore::rt::Event;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// #[derive(Serialize, Deserialize)]
// struct GreetArgs<'a> {
//     name: &'a str,
// }

#[component]
pub fn App<G: Html>(cx: Scope) -> View<G> {
    let name = create_signal(cx, String::new());
    let greet_msg = create_signal(cx, String::new());
    // let secret_number = create_signal(cx, 0);
    // [TENHO DE IMPLEMENTAR AQUI COMO ATUALIZAR secret_number ATRAVÉS DE get_random()]

    let greet = move |e: Event| {
        e.prevent_default();
        spawn_local_scoped(cx, async move {
            // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
            let new_msg =
                invoke("guess", to_value(&GreetArgs { name: &name.get() }).unwrap()).await;

            log(&new_msg.as_string().unwrap());

            greet_msg.set(new_msg.as_string().unwrap());
        })
    };

    view! { cx,
        main(class="container") {
            h1{ "Guess Game in Tauri" }
            div(class="row") {
                img(src="public/guess.png", class="logo guess", alt="Guess logo")
            }
            p {
                "Please input your guess."
            }
            form(class="row",on:submit=greet) {
                input(id="greet-input",bind:value=name,placeholder="Enter a number...")
                button(type="submit") {
                    "Guess"
                }
            }
            p {
                b {
                    (greet_msg.get())
                }
            }
        }
    }
}
