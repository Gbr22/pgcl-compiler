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

// (Round)
pub fn is_opening_bracket_round(node: &TreeNode) -> bool {
    node.is_token_type(TokenType::OpeningBracketRound)
}
pub fn is_closing_bracket_round(node: &TreeNode) -> bool {
    node.is_token_type(TokenType::ClosingBracketRound)
}

// {Curly}
pub fn is_opening_bracket_curly(node: &TreeNode) -> bool {
    node.is_token_type(TokenType::OpeningBracketCurly)
}
pub fn is_closing_bracket_curly(node: &TreeNode) -> bool {
    node.is_token_type(TokenType::ClosingBracketCurly)
}