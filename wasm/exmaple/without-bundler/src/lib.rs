use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    let window = web_sys::window().expect("there is no window in global");
    let document = window.document().expect("there is no document in window");
    let body = document.body().expect("there is no body in a document");

    let p_element = document
        .create_element("p")
        .expect("fail to create P element");

    p_element.set_inner_html("Hello from rust");

    body.append_child(&p_element)
        .expect("fail to append element");
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
