use wasm_bindgen::prelude::*;

use crate::analizer::AnalizeResult as RustAnalizeResult;

use super::tokenize::token_vec_into_js;
use super::tokenize::Token;

#[wasm_bindgen]
pub struct AnalizeResult {
    result: RustAnalizeResult,
}

#[wasm_bindgen]
impl AnalizeResult {
    #[wasm_bindgen(getter = tokens)]
    pub fn get_tokens(&self) -> Vec<Token> {
        token_vec_into_js(self.result.tokens.clone())
    }
}
