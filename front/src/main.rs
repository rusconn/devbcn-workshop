#![allow(non_snake_case)]
mod components;
mod models;

use dioxus::prelude::*;

use components::{FilmModal, Footer, Header};

fn main() {
    wasm_logger::init(wasm_logger::Config::default().module_prefix("front"));
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        main { class: "relative z-0 bg-blue-100 w-screen h-auto min-h-screen flex flex-col justify-start items-stretch",
            Header {}
            section { class: "md:container md:mx-auto md:py-8 flex-1" }
            Footer {}
            FilmModal { on_create_or_update: move |_| {}, on_cancel: move |_| {} }
        }
    })
}
