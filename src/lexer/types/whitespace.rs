use crate::lexer::types::token_type::TokenType;

pub fn get_whitespace() -> Vec<TokenType> {
    use TokenType as T;
    vec![T::Whitespace, T::Newline, T::InvalidChar, T::StartOfInput]
}

pub fn is_whitespace(typ: &TokenType) -> bool {
    get_whitespace().contains(&typ)
}
