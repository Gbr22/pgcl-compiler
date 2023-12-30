use crate::{lexer::types::token_type::TokenType, parser::tree::TreeNode};

pub fn get_brackets() -> Vec<TokenType> {
    use TokenType as T;
    vec![
        T::OpeningBracketRound,
        T::ClosingBracketRound,
        T::OpeningBracketSquare,
        T::ClosingBracketSquare,
        T::OpeningBracketCurly,
        T::ClosingBracketCurly,
    ]
}
