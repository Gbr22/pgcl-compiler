use super::token_def::TokenDef;

#[derive(Clone)]
pub struct CatchAllDef {}
impl TokenDef for CatchAllDef {
    fn get_priority(&self) -> i32 {
        -1
    }
    fn check_character(&self, _current: &str, _char: char) -> bool {
        true
    }

    fn is_valid(&self, r#_final: &str) -> bool {
        false
    }
}
