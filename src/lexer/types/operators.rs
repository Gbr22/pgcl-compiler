use crate::lexer::types::token_type::TokenType;

pub fn get_operators() -> Vec<TokenType> {
    use TokenType as T;
    vec![
        // number operators
        T::ForwardSlash,
        T::DoubleForwardSlash,
        T::Star,
        T::Dash,
        T::Plus,
        // logical operators
        T::Bang,
        // comparison operators
        T::DoubleEquals,
        T::BangEquals,
        T::LtEquals,
        T::GtEquals,
        T::Lt,
        T::Gt,
        // punctuation
        T::Colon,
        T::Comma,
        T::Dot,
        // other
        T::ArrowRight,
        T::Equals,
    ]
}
