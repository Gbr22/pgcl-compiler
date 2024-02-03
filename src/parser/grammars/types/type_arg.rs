use crate::parser::nodes::comma_separator::{find_next_type_separator_comma, find_next_value_separator_comma};

use crate::parser::parsers::types::type_arg::TypeArgParser;
use crate::parser::tree_nodes::TreeNodes;
use crate::use_parser;
use crate::{
    parser::parse::Parser,
    parser::{grammar::GrammarLike, tree::TreeNode},
};

pub struct TypeArgGrammar {}

impl GrammarLike for TypeArgGrammar {
    use_parser!(TypeArgParser);

    fn next_match_start(&self, nodes: &TreeNodes) -> Option<usize> {
        if nodes.len() == 0 {
            return None;
        }

        for (index, node) in nodes.iter().enumerate() {
            if let TreeNode::TypeArg(_) = node {
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
        let index = find_next_type_separator_comma(start_index, nodes.iter());

        match index {
            Some(index) => Some(index),
            None => Some(nodes.len() - 1),
        }
    }

    fn allow_parallel_processing(&self) -> bool {
        true
    }
}
