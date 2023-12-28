use wasm_bindgen::prelude::*;
pub mod tokens;
pub mod parser;

#[macro_use]
extern crate trait_enum;

#[wasm_bindgen]
pub fn hello_world(string: String) -> String {
    return format!("Hello {}!",string);
}