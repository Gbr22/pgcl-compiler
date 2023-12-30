use std::collections::VecDeque;

use serde_derive::Serialize;

use crate::{lexer::types::definitions::get_definition, common::range::Range};

use super::{types::token_type::TokenType, definitions::token_def::TokenDef};

#[derive(Clone, PartialEq, Serialize, Debug)]
pub struct Token {
    pub string: String,
    pub typ: TokenType,
    pub range: Range
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

    pub fn split_by_line(mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let mut parts: VecDeque<&str> = self.string.split('\n').collect();

        loop {
            let Some(str) = parts.pop_front() else {
                break;
            };

            let string = str.to_owned();
            let start_index = self.range.start_index;
            let end_index = start_index + string.chars().count();
            let range = Range::new(start_index, end_index);
            let typ = self.typ;

            tokens.push(Token {
                string,
                typ,
                range,
            });

            self.range.start_index = end_index + 1;
        }

        tokens
    }
}
