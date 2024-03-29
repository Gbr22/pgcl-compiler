use crate::lexer::types::keywords::FN;
use crate::parser::brackets::{curly_bracket, round_bracket};
use crate::parser::match_brackets::{find_bracket_end, SignedIndex};
use crate::parser::parsers::function_declaration::FunctionDeclarationParser;
use crate::parser::tree_nodes::TreeNodes;
use crate::use_parser;
use crate::{
    lexer::types::token_type::TokenType,
    parser::parse::Parser,
    parser::{grammar::GrammarLike, tree::TreeNode},
};

pub struct FunctionDeclarationGrammar {}

pub fn find_args_start<'a>(
    fn_index: usize,
    nodes: impl Iterator<Item = &'a TreeNode>,
) -> Option<usize> {
    for (index, node) in nodes.enumerate() {
        if index <= fn_index {
            continue;
        }
        if node.is_token_type(TokenType::OpeningBracketRound) {
            return Some(index);
        }
    }

    None
}
pub fn find_args_end<'a>(
    args_start_index: impl Into<SignedIndex>,
    nodes: impl Iterator<Item = &'a TreeNode>,
) -> Option<usize> {
    find_bracket_end(round_bracket(), args_start_index, nodes)
}
pub fn find_body_start<'a>(
    args_end_index: usize,
    nodes: impl Iterator<Item = &'a TreeNode>,
) -> Option<usize> {
    for (index, node) in nodes.enumerate() {
        if index <= args_end_index {
            continue;
        }
        if node.is_token_type(TokenType::OpeningBracketCurly) {
            return Some(index);
        }
    }

    None
}
pub fn find_body_end<'a>(
    body_start_index: usize,
    nodes: impl Iterator<Item = &'a TreeNode>,
) -> Option<usize> {
    find_bracket_end(curly_bracket(), body_start_index, nodes)
}

impl GrammarLike for FunctionDeclarationGrammar {
    use_parser!(FunctionDeclarationParser);

    fn next_match_start(&self, nodes: &TreeNodes) -> Option<usize> {
        for (index, node) in nodes.iter().enumerate() {
            if node.is_keyword(FN) {
                return Some(index);
            }
        }

        None
    }

    fn next_match_end(&self, nodes: &TreeNodes, start_index: usize) -> Option<usize> {
        let args_start = find_args_start(start_index, nodes.iter())?;
        let args_end = find_args_end(args_start, nodes.iter())?;
        let body_start = find_body_start(args_end, nodes.iter())?;
        let body_end = find_body_end(body_start, nodes.iter())?;

        Some(body_end)
    }

    fn allow_parallel_processing(&self) -> bool {
        true
    }
}
