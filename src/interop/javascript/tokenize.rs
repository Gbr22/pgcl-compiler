use wasm_bindgen::prelude::*;
use crate::lexer::token::Token as RustToken;
use crate::lexer::tokenize::tokenize as rust_tokenize;
use crate::lexer::types::keywords::get_keywords as rust_get_keywords;
use crate::lexer::types::token_type::TokenType;

#[wasm_bindgen(js_name = TokenizeResult)]
#[derive(Debug)]
pub struct TokenizeResult {
    pub todo: i32,
    tokens: Vec<RustToken>,
    failed_tokens: Vec<RustToken>
}

pub fn token_vec_into_js(tokens: Vec<RustToken>) -> Vec<Token> {
    let tokens: Vec<Token> = tokens.into_iter().map(|token|{
        let token_js: Token = token.to_owned().into();
        
        token_js
    }).collect();

    tokens
}


#[wasm_bindgen]
impl TokenizeResult {
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("{:?}",&self)
    }

    #[wasm_bindgen(getter = tokens)]
    pub fn get_tokens(&self) -> Vec<Token> {
        token_vec_into_js(self.tokens.clone())
    }
    #[wasm_bindgen(getter = failedTokens)]
    pub fn get_failed_tokens(&self) -> Vec<Token> {
        token_vec_into_js(self.failed_tokens.clone())
    }
}

#[wasm_bindgen(js_name = tokenize)]
pub fn tokenize(input: &str) -> TokenizeResult {
    let result = rust_tokenize(input);
    TokenizeResult {
        todo: 0,
        tokens: result.tokens,
        failed_tokens: result.failed_tokens
    }
}

impl Into<Token> for RustToken {
    fn into(self) -> Token {
        Token { token: self }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Token {
    token: RustToken
}
#[wasm_bindgen]
impl Token {
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("{:?}",&self)
    }

    #[wasm_bindgen(js_name = isValid)]
    pub fn is_valid(&self) -> bool {
        self.token.is_valid()
    }
    #[wasm_bindgen(getter = string)]
    pub fn get_string(&self) -> String {
        self.token.string.to_owned()
    }
    #[wasm_bindgen(getter = typ)]
    pub fn get_type(&self) -> TokenType {
        self.token.typ
    }
    #[wasm_bindgen(getter = startIndex)]
    pub fn get_start_index(&self) -> usize {
        self.token.start_index
    }
    #[wasm_bindgen(getter = endIndex)]
    pub fn get_end_index(&self) -> usize {
        self.token.end_index
    }
    #[wasm_bindgen(js_name = splitByLine)]
    pub fn split_by_line(self) -> Vec<Token> {
        let js_tokens: Vec<Token> = self.token.split_by_line().iter().map(|token| {
            let js_token: Token = token.clone().into();
            
            js_token
        }).collect();

        js_tokens
    }
}

#[wasm_bindgen(js_name = getKeywords)]
pub fn get_keywords() -> Vec<String> {
    let strings = rust_get_keywords()
        .iter()
        .map(|s|s.to_string())
        .collect();
    
    strings
}