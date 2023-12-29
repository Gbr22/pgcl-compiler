use crate::lexer::types::token_type::TokenType;

pub fn get_comments() -> Vec<TokenType> {
    use TokenType as T;
    vec![
        T::LineComment,
        T::BlockComment,
    ]
}