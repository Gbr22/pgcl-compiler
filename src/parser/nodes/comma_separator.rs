use crate::{parser::{brackets::{BracketType, curly_bracket, round_bracket, square_bracket}, match_brackets::{BracketTracker, find_next_match_outside_brackets}, tree::TreeNode}, lexer::types::token_type::TokenType};

fn is_comma(node: &TreeNode) -> bool {
    node.is_token_type(TokenType::Comma)
}

pub fn find_next_comma_outside_brackets(start_index: usize, nodes: &[TreeNode]) -> Option<usize> {
    find_next_match_outside_brackets(vec![
        curly_bracket(),
        round_bracket(),
        square_bracket()
    ], is_comma, start_index, nodes)
}