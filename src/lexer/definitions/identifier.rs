use super::token_def::TokenDef;

#[derive(Clone)]
pub struct IdentifierDef {}
impl TokenDef for IdentifierDef {
    fn get_priority(&self) -> i32 {
        0
    }
    fn check_character(&self, current: &str, char: char) -> bool {
        let new = format!("{}{}", current, char);

        Self::is_valid(self, &new)
    }

    fn is_valid(&self, r#final: &str) -> bool {
        let english_letters = "abcdefghijklmnopqrstuvwxyz";
        let allowed_alpha: Vec<char> = format!(
            "{}{}",
            english_letters,
            english_letters.to_ascii_uppercase()
        )
        .chars()
        .collect();
        let allowed_sym: Vec<char> = "_$".chars().collect();
        let allowed_numbers: Vec<char> = "0123456789".chars().collect();

        for (index, char) in r#final.chars().enumerate() {
            let is_valid = if index == 0 {
                vec![allowed_alpha.contains(&char), allowed_sym.contains(&char)]
                    .into_iter()
                    .any(|b| b)
            } else {
                vec![
                    allowed_alpha.contains(&char),
                    allowed_sym.contains(&char),
                    allowed_numbers.contains(&char),
                ]
                .into_iter()
                .any(|b| b)
            };

            if !is_valid {
                return false;
            }
        }

        true
    }
}
