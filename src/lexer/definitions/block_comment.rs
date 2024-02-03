use super::token_def::TokenDef;

#[derive(Clone)]
pub struct BlockCommentDef {}
impl TokenDef for BlockCommentDef {
    fn get_priority(&self) -> i32 {
        -1
    }
    fn check_character(&self, current: &str, char: char) -> bool {
        if current.len() == 0 {
            return char == '/';
        }
        if current.len() == 1 {
            return char == '*';
        }

        let did_end = current.ends_with("*/");

        !did_end
    }

    fn is_valid(&self, str: &str) -> bool {
        str.starts_with("/*") && str.ends_with("*/")
    }
}
