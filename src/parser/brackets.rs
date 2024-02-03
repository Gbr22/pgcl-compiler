use super::tree::TreeNode;
use crate::lexer::types::token_type::TokenType;

pub struct BracketType {
    pub is_opening: Box<dyn Fn(&TreeNode) -> bool>,
    pub is_closing: Box<dyn Fn(&TreeNode) -> bool>,
}

// (Round)
pub fn is_opening_bracket_round(node: &TreeNode) -> bool {
    node.is_token_type(TokenType::OpeningBracketRound)
}
pub fn is_closing_bracket_round(node: &TreeNode) -> bool {
    node.is_token_type(TokenType::ClosingBracketRound)
}
pub fn round_bracket() -> BracketType {
    BracketType {
        is_opening: Box::new(is_opening_bracket_round),
        is_closing: Box::new(is_closing_bracket_round),
    }
}

// {Curly}
pub fn is_opening_bracket_curly(node: &TreeNode) -> bool {
    node.is_token_type(TokenType::OpeningBracketCurly)
}
pub fn is_closing_bracket_curly(node: &TreeNode) -> bool {
    node.is_token_type(TokenType::ClosingBracketCurly)
}
pub fn curly_bracket() -> BracketType {
    BracketType {
        is_opening: Box::new(is_opening_bracket_curly),
        is_closing: Box::new(is_closing_bracket_curly),
    }
}

// [Square]
pub fn is_opening_bracket_square(node: &TreeNode) -> bool {
    node.is_token_type(TokenType::OpeningBracketSquare)
}
pub fn is_closing_bracket_square(node: &TreeNode) -> bool {
    node.is_token_type(TokenType::ClosingBracketSquare)
}
pub fn square_bracket() -> BracketType {
    BracketType {
        is_opening: Box::new(is_opening_bracket_square),
        is_closing: Box::new(is_closing_bracket_square),
    }
}
