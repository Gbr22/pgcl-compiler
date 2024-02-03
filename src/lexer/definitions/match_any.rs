use super::token_def::TokenDef;

#[derive(Clone)]
pub struct MatchAnyDef {
    pub chars: Vec<char>,
}
impl TokenDef for MatchAnyDef {
    fn get_priority(&self) -> i32 {
        0
    }
    fn check_character(&self, _current: &str, char: char) -> bool {
        self.chars.contains(&char)
    }

    fn is_valid(&self, r#final: &str) -> bool {
        r#final.chars().all(|char| self.chars.contains(&char))
    }
}
