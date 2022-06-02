use wasm_bindgen::prelude::*;

/**
 * Alert 정의
 */
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn big_computation() {
    alert("엄청나게 복잡한 연산 처리 중");
}

#[wasm_bindgen]
pub fn welcome(name: &str) {
   alert(&format!("Hello {}, from React with RUST WASM!", name));
}