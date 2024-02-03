use crate::parser::nodes::comma_separator::find_next_comma_outside_brackets;

use crate::parser::parsers::function_arg::FunctionArgParser;
use crate::parser::tree_nodes::TreeNodes;
use crate::use_parser;
use crate::{
    parser::parse::Parser,
    parser::{grammar::GrammarLike, tree::TreeNode},
};

pub struct FunctionArgGrammar {}

impl GrammarLike for FunctionArgGrammar {
    use_parser!(FunctionArgParser);

    fn next_match_start(&self, nodes: &TreeNodes) -> Option<usize> {
        if nodes.len() == 0 {
            return None;
        }

        for (index, node) in nodes.iter().enumerate() {
            if let TreeNode::FunctionArg(_) = node {
                continue;
            }
            if let TreeNode::ParseError(_) = node {
                continue;
            }

            return Some(index);
        }

        None
    }

    fn next_match_end(&self, nodes: &TreeNodes, start_index: usize) -> Option<usize> {
        let index = find_next_comma_outside_brackets(start_index, nodes.iter());

        match index {
            Some(index) => Some(index),
            None => Some(nodes.len() - 1),
        }
    }
}