pub mod js_glue;

// mod app;
// use app::App;

// mod guess;
// use guess::App;

// pub mod mouse_tracker;
// use mouse_tracker::*;
// use sycamore::view;

pub mod show_random;
use show_random::App;

fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(App);
    // sycamore::render(|cx| view! { cx, MouseTracker {} });
    // sycamore::render(|| view! { App {} });
}
