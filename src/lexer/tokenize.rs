

use serde_derive::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

use super::{token::{Token, TokenJs}, types::{definitions::{get_definitions, get_definition}, token_type::TokenType}};
use super::definitions::token_def::TokenDef;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn max_token(tokens: &Vec<&Token>) -> Option<Token> {
    let mut sorted = tokens.clone();
    sorted.sort_by(|a, b| {
        a.def().get_priority().cmp(&b.def().get_priority())
    });
    let max = sorted.iter()
        .max_by(|a, b| 
            a.string
                .chars()
                .count()
                .cmp(
                    &b.string.chars().count()
                )
        );
    if let Some(max) = max {
        Some(max.to_owned().clone())
    } else {
        None
    }
}

fn flush_tokens(state: &mut LexerState) -> Option<Token> {
    let old_tokens = state.current_tokens.clone();
    state.current_tokens.clear();
    
    let mut valid_tokens: Vec<&Token> = Vec::new();
    let mut invalid_tokens: Vec<&Token> = Vec::new();

    for token_state in &old_tokens {
        if token_state.token.is_valid() {
            valid_tokens.push(&token_state.token);
        } else {
            invalid_tokens.push(&token_state.token);
        }
    }
    
    let max_valid = max_token(&valid_tokens);

    if let Some(max) = max_valid {
        state.tokens.push(max.clone());

        Some(max)
    } else {
        let max_invalid = max_token(&invalid_tokens);
        if let Some(max) = max_invalid {
            state.failed_tokens.push(max);
        }

        None
    }

}

#[wasm_bindgen()]
pub struct TokenizeResultJs {
    tokens: Vec<Token>,
    failed_tokens: Vec<Token>
}

#[derive(Debug, Serialize)]
pub struct TokenizeResult {
    pub tokens: Vec<Token>,
    pub failed_tokens: Vec<Token>
}
#[wasm_bindgen(js_name = TokenizeResult)]
impl TokenizeResultJs {
    #[wasm_bindgen(getter = tokens)]
    pub fn get_tokens(&self) -> Vec<TokenJs> {
        let tokens: Vec<TokenJs> = self.tokens.iter().map(|token|{
            let token_js: TokenJs = token.to_owned().into();
            
            token_js
        }).collect();
        
        tokens
    }
    #[wasm_bindgen(getter = failedTokens)]
    pub fn get_failed_tokens(&self) -> Vec<TokenJs> {
        let tokens: Vec<TokenJs> = self.failed_tokens.iter().map(|token|{
            let token_js: TokenJs = token.to_owned().into();
            
            token_js
        }).collect();
        
        tokens
    }
}

#[derive(Clone, Debug)]
pub struct TokenState {
    token: Token,
    is_finished: bool
}
impl TokenState {
    fn into_token(self) -> Token {
        self.token
    }
}

#[wasm_bindgen(js_name = tokenize)]
pub fn tokenize_js(input: &str) -> TokenizeResultJs {
    let result = tokenize(input);
    TokenizeResultJs {
        tokens: result.tokens,
        failed_tokens: result.failed_tokens
    }
}

struct LexerState {
    pub tokens: Vec<Token>,
    pub failed_tokens: Vec<Token>,
    pub current_tokens: Vec<TokenState>,
    pub index: usize
}

pub fn tokenize(input: &str) -> TokenizeResult {
    let mut state = LexerState {
        tokens: vec![],
        failed_tokens: vec![],
        current_tokens: vec![],
        index: 0
    };

    state.current_tokens.push(TokenState { token: Token {
        string: "".to_owned(),
        typ: TokenType::StartOfInput,
        start_index: 0,
        end_index: 0
    }, is_finished: true });
    
    let input_chars: Vec<char> = input.chars().collect();

    loop {
        if state.index >= input_chars.len() {
            if state.current_tokens.len() == 0 {
                break;
            }
            let token = flush_tokens(&mut state);
            if let Some(token) = token {
                state.index = token.end_index;
            } else {
                let failed_token = state.failed_tokens.last().unwrap().to_owned();
                let sub_token = failed_token.def().largest_valid_subtoken(&failed_token);
                if let Some(sub_token) = sub_token {
                    // unroll
                    state.index = sub_token.end_index;
                    state.failed_tokens.pop();
                    state.current_tokens.push(TokenState {
                        token: sub_token,
                        is_finished: true
                    });
                }
            }
            continue;
        }
        let char = input_chars[state.index];
        if state.current_tokens.len() == 0 {
            for typ in get_definitions() {
                let def = get_definition(typ);

                if !def.check_character("",char) {
                    continue;
                }

                let token = Token {
                    typ: typ.to_owned(),
                    string: format!("{}",char),
                    start_index: state.index,
                    end_index: state.index + 1
                };

                state.current_tokens.push(TokenState { token, is_finished: false });
            }
            state.index = state.index + 1;
            continue;
        }


        let mut did_extend = false;
        state.current_tokens = state.current_tokens.iter().map(|token_state: &TokenState| {
            let token = &token_state.token;
            let is_finished = token_state.is_finished;
            let def = token.def();
            let can_extend = def.check_character(&token.string,char);
            if can_extend && !is_finished {
                did_extend = true;

                TokenState {
                    token: Token {
                        typ: token.typ,
                        string: format!("{}{}",token.string.to_owned(),char),
                        start_index: token.start_index,
                        end_index: state.index + 1
                    },
                    is_finished
                }
            } else {
                TokenState {
                    token: token.clone(),
                    is_finished: true
                }
            }
        }).collect();

        if did_extend {
            state.index = state.index + 1;
            continue;
        }

        let token = flush_tokens(&mut state);
        if let Some(token) = token {
            state.index = token.end_index;
        } else {
            let failed_token = state.failed_tokens.last().unwrap().to_owned();
            let sub_token = failed_token.def().largest_valid_subtoken(&failed_token);
            if let Some(sub_token) = sub_token {
                // unroll
                state.index = sub_token.end_index;
                state.failed_tokens.pop();
                state.current_tokens.push(TokenState {
                    token: sub_token,
                    is_finished: true
                });
            }
        }
    }

    TokenizeResult {
        tokens: state.tokens,
        failed_tokens: state.failed_tokens
    }
}