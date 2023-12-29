use super::token_def::TokenDef;

#[derive(Clone)]
pub struct LineCommentDef {}
impl TokenDef for LineCommentDef {
    fn get_priority(&self) -> i32 { -1 }
    fn check_character(&self, current: &str, char: char) -> bool {
        if current.len() <= 1 {
            return char == '/';
        }

        char != '\n' && char != '\r'
    }

    fn is_valid(&self, str: &str) -> bool {
        if !str.starts_with("//") {
            return false;
        }
        if str.contains('\n') || str.contains('\r') {
            return  false;
        }

        true
    }
}