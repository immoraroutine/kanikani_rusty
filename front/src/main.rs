#![allow(non_snake_case)]
// Import the Dioxus prelude to gain access to the `rsx!` macro and the `Scope` and `Element` types.
mod components;
mod models;

use components::{FilmModal, Footer, Header};
use dioxus::prelude::*;
use models::FilmModalVisibility;

fn main() {
    // Launch the web application using the App component as the root.
    dioxus_web::launch(App);
}

// Define a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || FilmModalVisibility(false));
    let is_modal_visible = use_shared_state::<FilmModalVisibility>(cx).unwrap();
    cx.render(rsx! {
       div {
           "Hello, world!"
       }
       main {
           class: "relative z-0 bg-blue-100 w-screen h-auto min-h-screen flex flex-col justify-start items-stretch",
           Header {}
           section {
               class: "md:container md:mx-auto md:py-8 flex-1",
           }
            Footer {}
            FilmModal {
                on_create_or_update: move |_| {},
                on_cancel: move |_| {
                    is_modal_visible.write().0 = false;
                },
           }
       }
    })
}
