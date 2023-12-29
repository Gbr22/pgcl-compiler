use wasm_bindgen::prelude::*;

use crate::analizer::analize as rust_analize;
use crate::analizer::AnalizeResult as RustAnalizeResult;

use super::tokenize::Token;
use super::tokenize::token_vec_into_js;

#[wasm_bindgen]
pub struct AnalizeResult {
    result: RustAnalizeResult
}

#[wasm_bindgen]
impl AnalizeResult {
    #[wasm_bindgen(getter = tokens)]
    pub fn get_tokens(&self) -> Vec<Token> {
        token_vec_into_js(self.result.tokens.clone())
    }
}

#[wasm_bindgen]
pub fn analize(input: &str) -> AnalizeResult {
    let result = rust_analize(input);

    AnalizeResult {
        result
    }
}