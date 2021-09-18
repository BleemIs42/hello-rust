#![feature(proc_macro)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use web_sys::{self, Document, Element, HtmlElement, Window};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn run_alert(item: &str) {
    alert(&format!("This is a WASM and {}", item))
}

#[wasm_bindgen]
pub fn create_stuff() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let div = document.create_element("div").expect("document create a div element");
    let p = document.create_element("p").expect("document create a p element");

    p.set_text_content(Some("Hello from WASM in Rust!"));
    div.append_child(&p);

    body.append_child(&div);
}
