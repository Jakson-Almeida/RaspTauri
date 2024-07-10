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

#[derive(Serialize, Deserialize)]
struct GuessArgs<'a> {
    name: &'a str,
    // secret_number: u32,
}

#[component]
pub fn App<G: Html>(cx: Scope) -> View<G> {
    let name = create_signal(cx, String::new());
    let greet_msg = create_signal(cx, String::new());
    let secret_number = create_signal(cx, 0);

    // Função para obter um número aleatório
    let get_random_number = move |_: Event| {
        spawn_local_scoped(cx, async move {
            let random_number: u32 = invoke("get_random", JsValue::NULL)
                .await
                .as_f64()
                .unwrap() as u32;
            secret_number.set(random_number);
            log(&format!("Random number generated: {}", random_number));
        });
    };

    // Inicializa `secret_number` com um número aleatório na montagem do componente
    create_effect(cx, move || {
        spawn_local_scoped(cx, async move {
            let random_number: u32 = invoke("get_random", JsValue::NULL)
                .await
                .as_f64()
                .unwrap() as u32;
            secret_number.set(random_number);
            log(&format!("Initial random number: {}", random_number));
        });
    });

    let greet = move |e: Event| {
        e.prevent_default();
        let name_value = name.get().to_string();
        let secret_number_value = *secret_number.get();
        log("Estou em greet");
        spawn_local_scoped(cx, async move {
            let args = GuessArgs {
                name: &name_value,
                // secret_number: secret_number_value,
            };
            log("\"Invocar\" os parametros");
            let new_msg = invoke("guess2", to_value(&args).unwrap()).await;
            // let new_msg = format!("É um teste");
            log("Parametros \"invocados\"");
            log(&new_msg.as_string().unwrap());
            greet_msg.set(new_msg.as_string().unwrap());
            log("greet_msg recebeu o valor");
        });
    };

    log("view!");

    view! { cx,
        main(class="container") { //style="background-image: url('public/ground.png'); background-repeat: repeat;"
            h1 { "Guess Game in Tauri" }
            div(class="row") {
                img(src="public/guess.png", class="logo guess", alt="Guess logo")
            }
            p { "Please input your guess." }
            form(class="row", on:submit=greet) {
                input(id="greet-input", bind:value=name, placeholder="Enter a number...")
                button(type="submit") { "Guess" }
            }
            p { b { (greet_msg.get()) } }
            // p { b { (name.get().to_string()) } }
            // button(on:click=get_random_number) { "New Random Number" }
            // p { "Secret number: " (secret_number.get()) }
        }
    }
}
