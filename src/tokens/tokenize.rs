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
#[derive(Clone, PartialEq, Serialize)]
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

        def.is_valid(self.string.to_owned())
    }

    #[wasm_bindgen(getter = string)]
    pub fn js_get_string(&self) -> String {
        self.string.to_owned()
    }
}

pub trait TokenDef {
    fn check_character(&self, index: usize, char: u8, string: String) -> bool;
    fn is_valid(&self, r#final: String) -> bool;
}

#[derive(Clone)]
pub struct IdentifierDef {}
impl TokenDef for IdentifierDef {
    fn check_character(&self, _: usize, char: u8, _: String) -> bool {
        let re = Regex::new(r"[\w_]").expect("Regex should be valid!");
        
        re.is_match(&char.to_string())
    }

    fn is_valid(&self, r#final: String) -> bool {
        let re = Regex::new(r"[\w_]+").expect("Regex should be valid!");
        
        re.is_match(&r#final)
    }
}

#[derive(Clone)]
pub struct MatchAnyDef {
    bytes: Vec<u8>
}
impl TokenDef for MatchAnyDef {
    fn check_character(&self, index: usize, char: u8, _string: String) -> bool {
        index == 0 && self.bytes.contains(&char)
    }

    fn is_valid(&self, r#final: String) -> bool {
        r#final
            .as_bytes()
            .iter()
            .all(|b| self.bytes.contains(b))
    }
    
}

#[derive(Clone)]
pub struct NumberDef {}
impl TokenDef for NumberDef {
    fn check_character(&self, index: usize, char: u8, _string: String) -> bool {
        let re = Regex::new(r"\d").expect("Regex should be valid!");
        if index == 0 {
            return re.is_match(&char.to_string());
        }
        else {
            return re.is_match(&char.to_string()) || char == b'.';
        }
    }
    fn is_valid(&self, r#final: String) -> bool {
        let re = Regex::new(r"\d+").expect("Regex should be valid!");
        re.is_match(&r#final)
    }
}

#[derive(Clone)]
pub struct ExactMatchDef {
    string: String
}
impl TokenDef for ExactMatchDef {
    fn check_character(&self, index: usize, char: u8, _string: String) -> bool {
        let bytes = self.string.as_bytes();

        if index >= bytes.len() {
            return false;
        }

        let at_index: u8 = bytes[index];
        
        at_index == char
    }

    fn is_valid(&self, r#final: String) -> bool {
        r#final == self.string
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
    Number,
}

fn get_definitions() -> &'static [TokenType] {
    TokenType::all_variants()
}

fn get_definition(typ: &TokenType) -> Box<dyn TokenDef> {
    use TokenType as T;
    match typ {
        T::StartOfInput=>ExactMatchDef { string: "".to_owned() }.into(),
        T::Identifier=>IdentifierDef {}.into(),
        T::Whitespace=>MatchAnyDef { bytes: " \n\t\r".to_owned().into_bytes() }.into(),
        T::Number=>NumberDef {}.into()
    }
}

#[wasm_bindgen]
pub fn tokenize(input: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut current_tokens: Vec<Token> = vec![];

    current_tokens.push(Token {
        string: "".to_owned(),
        typ: TokenType::StartOfInput,
        start_index: 0,
        end_index: 0
    });

    for (i, char) in input.as_bytes().iter().enumerate() {
        let mut has_match = false;
        let mut remove_list: Vec<Token> = vec![];

        log(&format!("Char: {} {}",i,char));

        let mut j = 0;
        while j < current_tokens.len() {
            let current_def = get_definition(&current_tokens[j].typ);

            if current_def.check_character(
                current_tokens[j].string.len(),
                *char,
                current_tokens[j].string.to_owned()
            ) {
                current_tokens[j].string = format!("{}{}",current_tokens[j].string,String::from_utf8(vec![*char]).unwrap_or("e".to_owned()));
                has_match = true;
                log(&format!("Current: {}",current_tokens[j].string));
            } else {
                current_tokens[j].end_index = i;
                let clone = current_tokens[j].clone();
                remove_list.push(clone);
            }

            j=j+1;
        }

        if current_tokens.len() == remove_list.len() {
            log(&format!("Len matches {}",current_tokens.len()));
            if remove_list.iter().any(|t| t.is_valid()) {
                let mut iter = remove_list.iter().take_while(|t| t.is_valid());
                if let Some(first) = iter.next() {
                    tokens.push(first.clone());
                }
            }
        }

        current_tokens = current_tokens
            .iter()
            .skip_while(|t| remove_list.contains(t))
            .map(|t|t.to_owned())
            .collect();

        if !has_match {
            for token_type in get_definitions().iter() {
                let def = get_definition(token_type);
                if def.check_character(0, *char, "".to_owned()) {
                    log(&format!("Potential token: {:?} c:{} i:{}",token_type, char, i));
                    current_tokens.push(Token {
                        typ: token_type.to_owned(),
                        string: String::from_utf8(vec![*char]).unwrap_or("e".to_string()),
                        start_index: i,
                        end_index: i+1, 
                    })
                }
            }
        }
    }

    return tokens;
}