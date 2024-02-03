use crate::parser::brackets::{is_closing_bracket_angle, is_opening_bracket_angle};
use crate::parser::nodes::comma_separator::{find_next_type_separator_comma, find_next_value_separator_comma};

use crate::parser::parsers::types::compound::CompoundTypeParser;
use crate::parser::parsers::types::type_arg::TypeArgParser;
use crate::parser::tree_nodes::TreeNodes;
use crate::use_parser;
use crate::{
    parser::parse::Parser,
    parser::{grammar::GrammarLike, tree::TreeNode},
};

pub struct CompoundTypeGrammar {}

impl GrammarLike for CompoundTypeGrammar {
    use_parser!(CompoundTypeParser);

    fn next_match_start(&self, nodes: &TreeNodes) -> Option<usize> {
        if nodes.len() == 0 {
            return None;
        }

        for (index, node) in nodes.iter().enumerate() {
            if is_opening_bracket_angle(node) && index >= 1 {
                return Some(index-1);
            }
        }

        None
    }

    fn next_match_end(&self, nodes: &TreeNodes, start_index: usize) -> Option<usize> {
        Some(nodes.len() - 1)
    }

    fn allow_parallel_processing(&self) -> bool {
        true
    }
}
