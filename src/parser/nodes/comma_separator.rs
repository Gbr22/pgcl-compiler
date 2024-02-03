use crate::{
    lexer::types::token_type::TokenType,
    parser::{
        brackets::{angle_bracket, curly_bracket, round_bracket, square_bracket},
        match_brackets::find_next_match_outside_brackets,
        tree::TreeNode,
    },
};

fn is_comma(node: &TreeNode) -> bool {
    node.is_token_type(TokenType::Comma)
}

pub fn find_next_value_separator_comma<'a>(
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

pub fn find_next_type_separator_comma<'a>(
    start_index: usize,
    nodes: impl Iterator<Item = &'a TreeNode>,
) -> Option<usize> {
    find_next_match_outside_brackets(
        vec![
            curly_bracket(),
            round_bracket(),
            square_bracket(),
            angle_bracket(),
        ],
        is_comma,
        start_index,
        nodes,
    )
}
