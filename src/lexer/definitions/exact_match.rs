use super::token_def::TokenDef;

#[derive(Clone)]
pub struct ExactMatchDef {
    pub string: String
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