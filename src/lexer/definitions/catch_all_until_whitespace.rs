use super::token_def::TokenDef;

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