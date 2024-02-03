use crate::lexer::token::Token;

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
            clone.range.end_index = clone.range.end_index - 1;
        }
    }
}

impl<T: TokenDef + Clone + 'static> From<T> for Box<dyn TokenDef> {
    fn from(value: T) -> Self {
        Box::new(value)
    }
}
