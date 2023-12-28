use super::token_def::TokenDef;

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