

use serde_derive::{Serialize, Deserialize};
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
    pub fn get_string(&self) -> String {
        self.string.to_owned()
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

pub trait TokenDef {
    fn check_character(&self, current: &str, char: char) -> bool;
    fn is_valid(&self, r#final: &str) -> bool;
    fn get_priority(&self) -> i32;
    fn get_error_message(&self, _str: &str) -> Option<String> {
        None
    }
    fn largest_valid_subtoken(&self, token: &Token) -> Option<Token> {
        let mut clone: Token = token.clone();
        loop {
            if clone.is_valid() {
                return Some(clone);
            }
            if clone.string.chars().count() == 0 {
                return None;
            }
            clone.string.pop();
            clone.end_index = clone.end_index - 1;
        }
    }
}

#[derive(Clone)]
pub struct IdentifierDef {}
impl TokenDef for IdentifierDef {
    fn get_priority(&self) -> i32 { 0 }
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
    fn get_priority(&self) -> i32 { 0 }
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
    fn get_priority(&self) -> i32 { 0 }
    fn check_character(&self, current: &str, char: char) -> bool {
        let allowed_chars: Vec<char> = "0123456789.".chars().collect();
        if !allowed_chars.contains(&char) {
            return false;
        }

        let new = format!("{}{}",current,char);
        let mut dot_count = 0;
        for c in current.chars() {
            if c == '.' {
                dot_count = dot_count + 1;
            }
        }
        if dot_count >= 1 && char == '.' {
            return false;
        }

        if new.chars().collect::<Vec<char>>()[0] == '.' {
            return false;
        }

        true        
    }
    fn is_valid(&self, r#final: &str) -> bool {
        self.get_error_message(r#final).is_none()
    }
    fn get_error_message(&self, str: &str) -> Option<String> {
        let allowed_chars: Vec<char> = "0123456789.".chars().collect();
        let mut dot_count = 0;
        for char in str.chars() {
            if !allowed_chars.contains(&char) {
                return Some(format!("Number literal {:?} may only contain digits [0..9] and optionally a dot.",&str));
            }
            if char == '.' {
                dot_count = dot_count + 1;
            }
        };
        if dot_count > 1 {
            return Some(format!("Number literal {:?} may contain at most 1 dot.",&str));
        };
        let chars: Vec<char> = str.chars().collect();
        if chars.first() == Some(&'.') {
            return Some(format!("Number literal {:?} must not start with a dot.",&str));
        };
        if chars.last() == Some(&'.') {
            return Some(format!("Number literal {:?} must not end with a dot.",&str));
        };

        None
    }
}

#[derive(Clone)]
pub struct ExactMatchDef {
    string: String
}
impl TokenDef for ExactMatchDef {
    fn get_priority(&self) -> i32 { 0 }
    fn check_character(&self, current: &str, char: char) -> bool {
        let new = format!("{}{}",current,char);
        let new_char_count = new.chars().count();

        if new_char_count > self.string.chars().count() {
            return false;
        }

        let partial: String = self.string.chars().take(new_char_count).collect();

        partial.eq(&new)
    }

    fn is_valid(&self, r#final: &str) -> bool {
        r#final.eq(&self.string)
    }
}

#[derive(Clone)]
pub struct CatchAllDef {}
impl TokenDef for CatchAllDef {
    fn get_priority(&self) -> i32 { -1 }
    fn check_character(&self, _current: &str, _char: char) -> bool {
        true
    }

    fn is_valid(&self, r#_final: &str) -> bool {
        false
    }
}

#[derive(Clone)]
pub struct InvalidCharDef {}
impl TokenDef for InvalidCharDef {
    fn get_priority(&self) -> i32 { -1 }
    fn check_character(&self, current: &str, _char: char) -> bool {
        current.chars().count() == 0
    }

    fn is_valid(&self, r#_final: &str) -> bool {
        false
    }
    fn get_error_message(&self, str: &str) -> Option<String> {
        Some(format!("Invalid character {:?}",str))
    }
}

#[derive(Clone)]
pub struct CatchAllUntilWhitespaceDef {}
impl TokenDef for CatchAllUntilWhitespaceDef {
    fn get_priority(&self) -> i32 { -1 }
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
    InvalidChar,
    Semicolon,
    Colon,
    Comma,
    Dot,
    ForwardSlash,
    DoubleForwardSlash,
    Star,
    Dash,
    Plus,
    Bang,
    Equals,
    DoubleEquals,
    BangEquals,
    LtEquals,
    GtEquals,
    Lt,
    Gt,
    ArrowRight,
    OpeningBracketRound,
    ClosingBracketRound,
    OpeningBracketSquare,
    ClosingBracketSquare,
    OpeningBracketCurly,
    ClosingBracketCurly,
}

pub fn get_definitions() -> &'static [TokenType] {
    TokenType::all_variants()
}

pub fn get_keywords() -> Vec<&'static str> {
    vec![
        "let",
        "mut",
        "struct",
        "fn",
        "uniform",
        "return"
    ]
}

pub fn get_brackets() -> Vec<TokenType> {
    use TokenType as T;
    vec![
        T::OpeningBracketRound,
        T::ClosingBracketRound,
        T::OpeningBracketSquare,
        T::ClosingBracketSquare,
        T::OpeningBracketCurly,
        T::ClosingBracketCurly,
    ]
}

pub fn get_operators() -> Vec<TokenType> {
    use TokenType as T;
    vec![
        // number operators
        T::ForwardSlash,
        T::DoubleForwardSlash,
        T::Star,
        T::Dash,
        T::Plus,

        // logical operators
        T::Bang,

        // comparison operators
        T::DoubleEquals,
        T::BangEquals,
        T::LtEquals,
        T::GtEquals,
        T::Lt,
        T::Gt,

        // punctuation
        T::Colon,
        T::Comma,
        T::Dot,

        // other
        T::ArrowRight,
        T::Equals,
    ]
}

fn get_definition(typ: &TokenType) -> Box<dyn TokenDef> {
    use TokenType as T;
    match typ {
        // special
        T::StartOfInput=>ExactMatchDef { string: "".to_owned() }.into(),
        T::InvalidChar=>InvalidCharDef {}.into(),
        
        // whitespace
        T::Whitespace=>MatchAnyDef { chars: " \t\r".chars().collect() }.into(),
        T::Newline=>ExactMatchDef { string: "\n".to_owned() }.into(),
        
        // number operators
        T::ForwardSlash=>ExactMatchDef { string: "/".into() }.into(),
        T::DoubleForwardSlash=>ExactMatchDef { string: "//".into() }.into(),
        T::Star=>ExactMatchDef { string: "*".into() }.into(),
        T::Dash=>ExactMatchDef { string: "-".into() }.into(),
        T::Plus=>ExactMatchDef { string: "+".into() }.into(),

        // logical operators
        T::Bang=>ExactMatchDef { string: "!".into() }.into(),

        // comparison operators
        T::DoubleEquals=>ExactMatchDef { string: "==".into() }.into(),
        T::BangEquals=>ExactMatchDef { string: "!=".into() }.into(),
        T::LtEquals=>ExactMatchDef { string: "<=".into() }.into(),
        T::GtEquals=>ExactMatchDef { string: ">=".into() }.into(),
        T::Lt=>ExactMatchDef { string: "<".into() }.into(),
        T::Gt=>ExactMatchDef { string: ">".into() }.into(),

        // brackets
        T::OpeningBracketRound=>ExactMatchDef { string: "(".into() }.into(),
        T::ClosingBracketRound=>ExactMatchDef { string: ")".into() }.into(),
        T::OpeningBracketSquare=>ExactMatchDef { string: "[".into() }.into(),
        T::ClosingBracketSquare=>ExactMatchDef { string: "]".into() }.into(),
        T::OpeningBracketCurly=>ExactMatchDef { string: "{".into() }.into(),
        T::ClosingBracketCurly=>ExactMatchDef { string: "}".into() }.into(),

        // punctuation
        T::Semicolon=>ExactMatchDef { string: ";".into() }.into(),
        T::Colon=>ExactMatchDef { string: ":".into() }.into(),
        T::Comma=>ExactMatchDef { string: ",".into() }.into(),
        T::Dot=>ExactMatchDef { string: ".".into() }.into(),
        
        // other
        T::Identifier=>IdentifierDef {}.into(),
        
        // values
        T::Number=>NumberDef {}.into(),
        
        // symbols
        T::ArrowRight=>ExactMatchDef { string: "->".into() }.into(),
        T::Equals=>ExactMatchDef { string: "=".into() }.into(),
    }
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

fn flush_tokens(current_tokens: &mut Vec<TokenState>, tokens: &mut Vec<Token>, failed_tokens: &mut Vec<Token>) -> Option<Token> {
    let old_tokens = current_tokens.clone();
    current_tokens.clear();
    
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
        tokens.push(max.clone());

        Some(max)
    } else {
        let max_invalid = max_token(&invalid_tokens);
        if let Some(max) = max_invalid {
            failed_tokens.push(max);
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
    pub fn get_tokens(&self) -> Vec<Token> {
        self.tokens.to_owned()
    }
    #[wasm_bindgen(getter = failedTokens)]
    pub fn get_failed_tokens(&self) -> Vec<Token> {
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

#[wasm_bindgen(js_name = tokenize)]
pub fn tokenize_js(input: &str) -> TokenizeResultJs {
    let result = tokenize(input);
    TokenizeResultJs {
        tokens: result.tokens,
        failed_tokens: result.failed_tokens
    }
}

pub fn tokenize(input: &str) -> TokenizeResult {
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
            } else {
                let failed_token = failed_tokens.last().unwrap().to_owned();
                let sub_token = failed_token.def().largest_valid_subtoken(&failed_token);
                if let Some(sub_token) = sub_token {
                    // unroll
                    index = sub_token.end_index;
                    failed_tokens.pop();
                    current_tokens.push(TokenState {
                        token: sub_token,
                        is_finished: true
                    });
                }
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
        } else {
            let failed_token = failed_tokens.last().unwrap().to_owned();
            let sub_token = failed_token.def().largest_valid_subtoken(&failed_token);
            if let Some(sub_token) = sub_token {
                // unroll
                index = sub_token.end_index;
                failed_tokens.pop();
                current_tokens.push(TokenState {
                    token: sub_token,
                    is_finished: true
                });
            }
        }
    }

    TokenizeResult {
        tokens,
        failed_tokens
    }
}