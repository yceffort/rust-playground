use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // js_namespace는 console을 할당했다.
    // 즉 log만 쓰면 console.log가 된다.
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // 여기는 console.log
    #[wasm_bindgen(js_namespace=console, js_name=log)]
    fn log_u32(a: u32);

    // 여기도 console.log
    #[wasm_bindgen(js_namespace=console, js_name=log)]
    fn log_strings(a: &str, b: &str);
}

macro_rules! console_log {
    // log 함수랑 연결된다.
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// rust extern으로 하는 방법
fn rust() {
    log("Hello yceffort!");
    log_u32(42);
    log_strings("Hello", "yceffort")
}

// macro
fn using_macro() {
    console_log!("Hello {}!", "yceffort");
    console_log!("Hello yceffort");
}

// websys library
fn using_web_sys() {
    use web_sys::console;

    console::log_1(&"Hello using web-sys".into());

    let js: JsValue = 4.into();

    console::log_2(&"Logging arbitrary values looks like".into(), &js);
}

#[wasm_bindgen(start)]
pub fn run() {
    rust();
    using_macro();
    using_web_sys();
}
