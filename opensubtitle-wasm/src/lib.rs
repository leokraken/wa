use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn hello_world_simple() -> String {
    "hello world".to_string()
}

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
pub fn run_js_value_struct(value: JsValue) -> Result<String, JsValue> {
    let typed: MyStruct= match value.into_serde(){
        Ok(res) => res,
        Err(_) => {return Err(JsValue::from("Error on deserialize object"));}
    };
    Ok(format!("hello world {} {}", typed.text, typed.number))
}

#[wasm_bindgen]
pub fn run_jsvalue_on_return() -> Result<JsValue, JsValue> {
    let typed = MyStruct {
        text: "text".to_string(),
        number: 1,
    };
    match JsValue::from_serde(&typed){
        Err(_) => Err(JsValue::from("Error create type")),
        Ok(res) => Ok(res),
    }
}