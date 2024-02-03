use super::token_def::TokenDef;

#[derive(Clone)]
pub struct NumberDef {}
impl TokenDef for NumberDef {
    fn get_priority(&self) -> i32 {
        0
    }
    fn check_character(&self, current: &str, char: char) -> bool {
        let allowed_chars: Vec<char> = "0123456789.".chars().collect();
        if !allowed_chars.contains(&char) {
            return false;
        }

        let new = format!("{}{}", current, char);
        let mut dot_count = 0;
        for c in current.chars() {
            if c == '.' {
                dot_count += 1;
            }
        }
        if dot_count >= 1 && char == '.' {
            return false;
        }

        if new.chars().collect::<Vec<char>>()[0] == '.' {
            return false;
        }

        true
    }
    fn is_valid(&self, r#final: &str) -> bool {
        self.get_error_message(r#final).is_none()
    }
    fn get_error_message(&self, str: &str) -> Option<String> {
        let allowed_chars: Vec<char> = "0123456789.".chars().collect();
        let mut dot_count = 0;
        for char in str.chars() {
            if !allowed_chars.contains(&char) {
                return Some(format!(
                    "Number literal {:?} may only contain digits [0..9] and optionally a dot.",
                    &str
                ));
            }
            if char == '.' {
                dot_count += 1;
            }
        }
        if dot_count > 1 {
            return Some(format!(
                "Number literal {:?} may contain at most 1 dot.",
                &str
            ));
        };
        let chars: Vec<char> = str.chars().collect();
        if chars.first() == Some(&'.') {
            return Some(format!(
                "Number literal {:?} must not start with a dot.",
                &str
            ));
        };
        if chars.last() == Some(&'.') {
            return Some(format!(
                "Number literal {:?} must not end with a dot.",
                &str
            ));
        };

        None
    }
}
