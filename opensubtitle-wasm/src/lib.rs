use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn hello_world(name: String) -> String {
    format!("hello world {}", name)
}


#[wasm_bindgen]
pub fn run_js_value_string(name: &JsValue) -> String {
    let name = name.as_string().expect("should be a string");
    format!("hello world {}", name)
}

#[derive(Serialize, Deserialize, Debug)]
struct MyStruct {
    pub text: String,
    pub number: i32,
}

#[wasm_bindgen]
pub fn run_js_value_struct(value: JsValue) -> String {
    let typed: MyStruct= value.into_serde().expect("should be a string");
    format!("hello world {} {}", typed.text, typed.number)
}