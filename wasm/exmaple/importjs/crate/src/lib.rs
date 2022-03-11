use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/defined-in-js.js")]
extern "C" {
    // name 함수 정의
    fn name() -> String;

    // 클래스 정의
    type MyClass;

    // 클래스에 new keyword를 constructor로 정의 
    #[wasm_bindgen(constructor)]
    fn new() -> MyClass;

    // getter
    #[wasm_bindgen(method, getter)]
    fn number(this: &MyClass) -> u32;

    // setter
    #[wasm_bindgen(method, setter)]
    fn set_number(this: &MyClass, number: u32) -> MyClass;

    // toString
    #[wasm_bindgen(method)]
    fn toString(this: &MyClass) -> String;
}

// lifted from the `console_log` example
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn run() {
    log(&format!("Hello from {}!", name())); // should output "Hello from Rust!"

    let x = MyClass::new();
    assert_eq!(x.number(), 42);
    x.set_number(10);
    log(&x.toString());
}