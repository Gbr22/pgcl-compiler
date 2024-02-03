use crate::parser::nodes::comma_separator::find_next_comma_outside_brackets;

use crate::parser::parsers::function_call_arg::FunctionCallArgParser;
use crate::parser::tree_nodes::TreeNodes;
use crate::use_parser;
use crate::{
    parser::parse::Parser,
    parser::{grammar::GrammarLike, tree::TreeNode},
};

pub struct FunctionCallArgGrammar {}

impl GrammarLike for FunctionCallArgGrammar {
    use_parser!(FunctionCallArgParser);

    fn next_match_start(&self, nodes: &TreeNodes) -> Option<usize> {
        if nodes.len() == 0 {
            return None;
        }

        for (index, node) in nodes.iter().enumerate() {
            if let TreeNode::FunctionCallArg(_) = node {
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
