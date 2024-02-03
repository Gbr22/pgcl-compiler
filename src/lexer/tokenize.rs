use serde_derive::Serialize;

use crate::common::range::Range;

use super::{
    token::Token,
    types::{
        definitions::{get_definition, get_definitions},
        token_type::TokenType,
    },
};

fn max_token(tokens: &Vec<&Token>) -> Option<Token> {
    let mut sorted = tokens.clone();
    sorted.sort_by(|a, b| a.def().get_priority().cmp(&b.def().get_priority()));
    let max = sorted
        .iter()
        .max_by(|a, b| a.string.chars().count().cmp(&b.string.chars().count()));
    if let Some(max) = max {
        Some(max.to_owned().clone())
    } else {
        None
    }
}

fn clear_and_take_best_token(state: &mut LexerState) -> Option<Token> {
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
        return Some(max);
    }

    let max_invalid = max_token(&invalid_tokens);
    if let Some(max) = max_invalid {
        return Some(max);
    }

    None
}

pub fn flush_tokens(state: &mut LexerState) {
    let maybe_token = clear_and_take_best_token(state);
    let Some(token) = maybe_token else {
        // nothing to flush
        return;
    };
    if token.is_valid() {
        let valid_token = token;
        state.index = valid_token.range.end_index;
        state.tokens.push(valid_token);
        return;
    }

    // attempt to recover failed token
    let failed_token = token;
    let valid_sub_token = failed_token.def().largest_valid_subtoken(&failed_token);
    let Some(valid_sub_token) = valid_sub_token else {
        // could not recover failed token
        state.failed_tokens.push(failed_token);
        return;
    };

    // successfully recovered a valid token
    state.index = valid_sub_token.range.end_index;
    state.tokens.push(valid_sub_token);
    return;
}

#[derive(Debug, Serialize)]
pub struct TokenizeResult {
    pub tokens: Vec<Token>,
    pub failed_tokens: Vec<Token>,
}

#[derive(Clone, Debug)]
pub struct TokenState {
    token: Token,
    is_finished: bool,
}
impl TokenState {
    fn into_token(self) -> Token {
        self.token
    }
}

pub struct LexerState {
    pub input_string: String,
    pub input_chars: Vec<char>,
    pub tokens: Vec<Token>,
    pub failed_tokens: Vec<Token>,
    pub current_tokens: Vec<TokenState>,
    pub index: usize,
}

impl LexerState {
    pub fn get_char(&self) -> char {
        self.input_chars[self.index]
    }
}

pub fn try_extend_tokens(state: &mut LexerState) -> bool {
    let char: char = *(&state.get_char());
    let mut did_extend = false;
    state.current_tokens = state
        .current_tokens
        .iter()
        .map(|token_state: &TokenState| {
            let token = &token_state.token;
            let is_finished = token_state.is_finished;
            let def = token.def();
            let can_extend = def.check_character(&token.string, char);
            if can_extend && !is_finished {
                did_extend = true;

                TokenState {
                    token: Token {
                        typ: token.typ,
                        string: format!("{}{}", token.string.to_owned(), char),
                        range: Range::new(token.range.start_index, state.index + 1),
                    },
                    is_finished,
                }
            } else {
                TokenState {
                    token: token.clone(),
                    is_finished: true,
                }
            }
        })
        .collect();

    if did_extend {
        state.index = state.index + 1;
    }

    did_extend
}

pub fn try_create_tokens(state: &mut LexerState) -> bool {
    if state.current_tokens.len() != 0 {
        return false;
    }
    let char = state.get_char();
    let mut did_create = false;
    for typ in get_definitions() {
        let def = get_definition(typ);

        if !def.check_character("", char) {
            continue;
        }

        let token = Token {
            typ: typ.to_owned(),
            string: format!("{}", char),
            range: Range::new(state.index, state.index + 1),
        };

        state.current_tokens.push(TokenState {
            token,
            is_finished: false,
        });
        did_create = true;
    }
    if did_create {
        state.index = state.index + 1;
    };

    did_create
}

pub enum LexerControlFlow {
    Continue,
    Break,
    FallThrough,
}

pub fn check_is_finished(state: &mut LexerState) -> LexerControlFlow {
    if state.index < state.input_chars.len() {
        return LexerControlFlow::FallThrough;
    }
    if state.current_tokens.len() == 0 {
        return LexerControlFlow::Break;
    }

    flush_tokens(state);

    LexerControlFlow::Continue
}

pub fn tokenize(input: &str) -> TokenizeResult {
    let mut state = LexerState {
        tokens: vec![],
        failed_tokens: vec![],
        current_tokens: vec![],
        index: 0,
        input_string: input.to_owned(),
        input_chars: input.chars().collect(),
    };

    state.current_tokens.push(TokenState {
        token: Token {
            string: "".to_owned(),
            typ: TokenType::StartOfInput,
            range: Range::null(),
        },
        is_finished: true,
    });

    loop {
        let flow = check_is_finished(&mut state);
        if let LexerControlFlow::Continue = flow {
            continue;
        }
        if let LexerControlFlow::Break = flow {
            break;
        }

        let did_create = try_create_tokens(&mut state);
        if did_create {
            continue;
        }

        let did_extend = try_extend_tokens(&mut state);
        if did_extend {
            continue;
        }

        flush_tokens(&mut state);
        continue;
    }

    TokenizeResult {
        tokens: state.tokens,
        failed_tokens: state.failed_tokens,
    }
}
