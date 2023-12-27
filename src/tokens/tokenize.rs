use std::fmt::format;

use serde_derive::Serialize;
use wasm_bindgen::prelude::*;

use enum_all_variants::AllVariants;
use regex::Regex;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
#[derive(Clone, PartialEq, Serialize, Debug)]
pub struct Token {
    string: String,
    pub typ: TokenType,
    pub start_index: usize,
    pub end_index: usize
}

#[wasm_bindgen]
impl Token {
    #[wasm_bindgen]
    pub fn is_valid(&self) -> bool {
        let def = get_definition(&self.typ);

        def.is_valid(&self.string)
    }

    fn def(&self) -> Box<dyn TokenDef> {
        get_definition(&self.typ)
    }

    #[wasm_bindgen(getter = string)]
    pub fn js_get_string(&self) -> String {
        self.string.to_owned()
    }
}

pub trait TokenDef {
    fn check_character(&self, current: &str, char: char) -> bool;
    fn is_valid(&self, r#final: &str) -> bool;
}

#[derive(Clone)]
pub struct IdentifierDef {}
impl TokenDef for IdentifierDef {
    fn check_character(&self, current: &str, char: char) -> bool {
        let new = format!("{}{}",current,char);

        Self::is_valid(&self, &new)
    }

    fn is_valid(&self, r#final: &str) -> bool {
        let english_letters = "abcdefghijklmnopqrstuvwxyz";
        let allowed_alpha: Vec<char> = format!("{}{}",english_letters,english_letters
            .to_ascii_uppercase())
            .chars().collect();
        let allowed_sym: Vec<char> = "_$".chars().collect();
        let allowed_numbers: Vec<char> = "0123456789".chars().collect();
        
        for (index, char) in r#final.chars().enumerate() {
            let is_valid = if index == 0 {
                vec![
                    allowed_alpha.contains(&char),
                    allowed_sym.contains(&char)
                ].into_iter().any(|b|b)
            } else {
                vec![
                    allowed_alpha.contains(&char),
                    allowed_sym.contains(&char),
                    allowed_numbers.contains(&char),
                ].into_iter().any(|b|b)
            };

            if !is_valid {
                return false;
            }
        }
        
        true
    }
}

#[derive(Clone)]
pub struct MatchAnyDef {
    chars: Vec<char>
}
impl TokenDef for MatchAnyDef {
    fn check_character(&self, _current: &str, char: char) -> bool {
        self.chars.contains(&char)
    }

    fn is_valid(&self, r#final: &str) -> bool {
        r#final
            .chars()
            .into_iter()
            .all(|char| self.chars.contains(&char))
    }
    
}

#[derive(Clone)]
pub struct NumberDef {}
impl TokenDef for NumberDef {
    fn check_character(&self, current: &str, char: char) -> bool {
        let allowed_chars: Vec<char> = "0123456789.".chars().collect();
        if !allowed_chars.contains(&char) {
            return false;
        }

        let new = format!("{}{}",current,char);
        let dot_count = new.chars()
            .take_while(|c|*c=='.')
            .count();

        if dot_count > 1 {
            return false;
        }

        if new.chars().collect::<Vec<char>>()[0] == '.' {
            return false;
        }

        true        
    }
    fn is_valid(&self, r#final: &str) -> bool {
        let re = Regex::new(r"^[0-9]+(.[0-9]+)?$").expect("Regex should be valid!");
        
        re.is_match(&r#final)
    }
}

#[derive(Clone)]
pub struct ExactMatchDef {
    string: String
}
impl TokenDef for ExactMatchDef {
    fn check_character(&self, current: &str, char: char) -> bool {
        let new = format!("{}{}",current,char);
        let new_chars: Vec<char> = new.chars().collect();
        let self_chars: Vec<char> = self.string.chars().collect();

        if new.chars().count() > self.string.chars().count() {
            return false;
        }

        for (index, char) in new_chars.iter().enumerate() {
            if self_chars[index] != *char {
                return false;
            }
        };

        true
    }

    fn is_valid(&self, r#final: &str) -> bool {
        r#final == self.string
    }
}

#[derive(Clone)]
pub struct CatchAllDef {}
impl TokenDef for CatchAllDef {
    fn check_character(&self, _current: &str, _char: char) -> bool {
        true
    }

    fn is_valid(&self, r#_final: &str) -> bool {
        false
    }
}

#[derive(Clone)]
pub struct CatchAllUntilWhitespaceDef {}
impl TokenDef for CatchAllUntilWhitespaceDef {
    fn check_character(&self, _current: &str, char: char) -> bool {
        let whitespace: Vec<char> = "\t \r\n".chars().collect();
        
        !whitespace.contains(&char)
    }

    fn is_valid(&self, r#_final: &str) -> bool {
        false
    }
}

impl<T: TokenDef + Clone + 'static> From<T> for Box<dyn TokenDef> {
    fn from(value: T) -> Self {
        Box::new(value)
    }
}

#[wasm_bindgen]
#[derive(AllVariants, Debug, Serialize, Clone, Copy, PartialEq)]
pub enum TokenType {
    StartOfInput,
    Identifier,
    Whitespace,
    Newline,
    Number,
    CatchAllUntilWhitespace
}

fn get_definitions() -> &'static [TokenType] {
    TokenType::all_variants()
}

fn get_definition(typ: &TokenType) -> Box<dyn TokenDef> {
    use TokenType as T;
    match typ {
        T::StartOfInput=>ExactMatchDef { string: "".to_owned() }.into(),
        T::Identifier=>IdentifierDef {}.into(),
        T::Whitespace=>MatchAnyDef { chars: " \t\r".chars().collect() }.into(),
        T::Newline=>ExactMatchDef { string: "\n".to_owned() }.into(),
        T::Number=>NumberDef {}.into(),
        T::CatchAllUntilWhitespace=>CatchAllUntilWhitespaceDef {}.into()
    }
}

fn max_token(tokens: &Vec<&Token>) -> Option<Token> {
    let max = tokens.iter()
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

fn flush_tokens(current_tokens: &mut Vec<TokenState>, tokens: &mut Vec<Token>, failed_tokens: &mut Vec<Token>) -> Option<Token> {
    let old_tokens = current_tokens.clone();
    current_tokens.clear();
    let valid_tokens: Vec<&Token> = old_tokens
        .iter()
        .map(|state|&state.token)
        .take_while(|token| token.is_valid()).collect();
    
    let max_valid = max_token(&valid_tokens);

    
    if let Some(max) = max_valid {
        tokens.push(max.clone());

        Some(max)
    } else {
        let invalid_tokens: Vec<&Token> = old_tokens
            .iter()
            .map(|state|&state.token)
            .skip_while(|token| token.is_valid())
            .collect();
        let max_invalid = max_token(&invalid_tokens);
        if let Some(max) = max_invalid {
            failed_tokens.push(max);
        }

        None
    }

}

#[wasm_bindgen]
pub struct TokenizeResult {
    tokens: Vec<Token>,
    failed_tokens: Vec<Token>
}
#[wasm_bindgen]
impl TokenizeResult {
    #[wasm_bindgen(getter = tokens)]
    pub fn js_get_tokens(&self) -> Vec<Token> {
        self.tokens.to_owned()
    }
    #[wasm_bindgen(getter = failedTokens)]
    pub fn js_get_failed_tokens(&self) -> Vec<Token> {
        self.failed_tokens.to_owned()
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

#[wasm_bindgen]
pub fn tokenize(input: String) -> TokenizeResult {
    let mut tokens: Vec<Token> = vec![];
    let mut current_tokens: Vec<TokenState> = vec![];
    let mut failed_tokens: Vec<Token> = vec![];

    current_tokens.push(TokenState { token: Token {
        string: "".to_owned(),
        typ: TokenType::StartOfInput,
        start_index: 0,
        end_index: 0
    }, is_finished: true });
    
    let input_chars: Vec<char> = input.chars().collect();
    let mut index: usize = 0;

    loop {
        if index >= input_chars.len() {
            if current_tokens.len() == 0 {
                break;
            }
            let token = flush_tokens(&mut current_tokens, &mut tokens, &mut failed_tokens);
            if let Some(token) = token {
                index = token.end_index;
            }
            continue;
        }
        let char = input_chars[index];
        if current_tokens.len() == 0 {
            for typ in get_definitions() {
                let def = get_definition(typ);

                if !def.check_character("",char) {
                    continue;
                }

                let token = Token {
                    typ: typ.to_owned(),
                    string: format!("{}",char),
                    start_index: index,
                    end_index: index + 1
                };

                current_tokens.push(TokenState { token, is_finished: false });
            }
            index = index + 1;
            continue;
        }


        let mut did_extend = false;
        current_tokens = current_tokens.iter().map(|state: &TokenState| {
            let token = &state.token;
            let is_finished = state.is_finished;
            let def = token.def();
            let can_extend = def.check_character(&token.string,char);
            if can_extend && !is_finished {
                did_extend = true;

                TokenState {
                    token: Token {
                        typ: token.typ,
                        string: format!("{}{}",token.string.to_owned(),char),
                        start_index: token.start_index,
                        end_index: index + 1
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
            index = index + 1;
            continue;
        }

        let token = flush_tokens(&mut current_tokens, &mut tokens, &mut failed_tokens);
        if let Some(token) = token {
            index = token.end_index;
            continue;
        }
    }

    TokenizeResult {
        tokens,
        failed_tokens
    }
}