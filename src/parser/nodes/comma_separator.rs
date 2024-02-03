use crate::{
    lexer::types::token_type::TokenType,
    parser::{
        brackets::{curly_bracket, round_bracket, square_bracket, BracketType},
        match_brackets::{find_next_match_outside_brackets, BracketTracker},
        tree::TreeNode,
    },
};

fn is_comma(node: &TreeNode) -> bool {
    node.is_token_type(TokenType::Comma)
}

pub fn find_next_comma_outside_brackets<'a>(
    start_index: usize,
    nodes: impl Iterator<Item = &'a TreeNode>,
) -> Option<usize> {
    find_next_match_outside_brackets(
        vec![curly_bracket(), round_bracket(), square_bracket()],
        is_comma,
        start_index,
        nodes,
    )
}
