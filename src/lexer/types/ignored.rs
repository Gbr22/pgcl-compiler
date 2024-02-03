use crate::lexer::types::{comments::get_comments, token_type::TokenType};

pub fn get_ignored_token_types() -> Vec<TokenType> {
    let mut vec: Vec<TokenType> = vec![];
    vec.extend(get_comments());
    use TokenType as T;
    vec.extend(vec![
        T::Whitespace,
        T::Newline,
        T::InvalidChar,
        T::StartOfInput,
    ]);

    vec
}

pub fn is_ignored_token_type(typ: &TokenType) -> bool {
    get_ignored_token_types().contains(typ)
}
