use serde_derive::Serialize;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::lexer::types::definitions::get_definition;

use super::{types::token_type::TokenType, definitions::token_def::TokenDef};

#[derive(Clone, PartialEq, Serialize, Debug)]
pub struct Token {
    pub string: String,
    pub typ: TokenType,
    pub start_index: usize,
    pub end_index: usize
}

#[wasm_bindgen(js_name = "Token")]
#[derive(Clone, Debug)]
pub struct TokenJs {
    token: Token
}
#[wasm_bindgen]
impl TokenJs {
    #[wasm_bindgen]
    pub fn is_valid(&self) -> bool {
        self.token.is_valid()
    }
    #[wasm_bindgen(getter = string)]
    pub fn get_string(&self) -> String {
        self.token.string.to_owned()
    }
    #[wasm_bindgen(getter = r#typ)]
    pub fn get_type(&self) -> TokenType {
        self.token.typ
    }
    #[wasm_bindgen(getter = r#startIndex)]
    pub fn get_start_index(&self) -> usize {
        self.token.start_index
    }
    #[wasm_bindgen(getter = r#endIndex)]
    pub fn get_end_index(&self) -> usize {
        self.token.end_index
    }
}

impl Into<TokenJs> for Token {
    fn into(self) -> TokenJs {
        TokenJs { token: self }
    }
}

impl Token {
    pub fn is_valid(&self) -> bool {
        let def = get_definition(&self.typ);

        def.is_valid(&self.string)
    }

    pub fn def(&self) -> Box<dyn TokenDef> {
        get_definition(&self.typ)
    }

    pub fn get_error_message(&self) -> String {
        let def = self.def();
        let is_valid = def.is_valid(&self.string);
        if is_valid {
            return format!("Token is valid. You should not see this.");
        }
        let msg = def.get_error_message(&self.string);
        let Some(msg) = msg else {
            return format!("Token ({:?}): {:?} is invalid.",self.typ,self.string);
        };
        
        msg
    }
}
