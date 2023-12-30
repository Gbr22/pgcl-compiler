use crate::lexer::types::keywords::FN;
use crate::parser::brackets::{round_bracket, curly_bracket};
use crate::parser::match_brackets::find_bracket_end;
use crate::parser::nodes::function_declaration::FunctionDeclaration;
use crate::{parser::{tree::TreeNode, grammar::GrammarLike}, lexer::types::{token_type::TokenType, keywords::UNIFORM}};

pub struct FunctionDeclarationGrammar {}

pub fn find_args_start(fn_index: usize, nodes: &[TreeNode]) -> Option<usize> {
    for (index, node) in nodes.iter().enumerate() {
        if index <= fn_index {
            continue;
        }
        if node.is_token_type(TokenType::OpeningBracketRound) {
            return Some(index)
        }
    }

    None
}
pub fn find_args_end(args_start_index: usize, nodes: &[TreeNode]) -> Option<usize> {
    find_bracket_end(
        round_bracket(),
        args_start_index,
        nodes
    )
}
pub fn find_body_start(args_end_index: usize, nodes: &[TreeNode]) -> Option<usize> {
    for (index, node) in nodes.iter().enumerate() {
        if index <= args_end_index {
            continue;
        }
        if node.is_token_type(TokenType::OpeningBracketCurly) {
            return Some(index)
        }
    }

    None
}
pub fn find_body_end(body_start_index: usize, nodes: &[TreeNode]) -> Option<usize> {
    find_bracket_end(
        curly_bracket(),
        body_start_index,
        nodes
    )
}

impl GrammarLike for FunctionDeclarationGrammar {
    fn next_match_start(&self, nodes: &[TreeNode]) -> Option<usize> {
        for (index, node) in nodes.iter().enumerate() {
            if node.is_keyword(FN) {
                return Some(index);
            }
        }

        None
    }

    fn next_match_end(&self, nodes: &[TreeNode], start_index: usize) -> Option<usize> {
        let args_start = find_args_start(start_index, &nodes)?;
        let args_end = find_args_end(args_start, &nodes)?;
        let body_start = find_body_start(args_end, &nodes)?;
        let body_end = find_body_end(body_start, &nodes)?;

        Some(body_end)
    }

    fn construct(&self, nodes: Vec<TreeNode>) -> TreeNode {
        FunctionDeclaration::parse(nodes)
    }
}