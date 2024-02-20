use crate::parser::brackets::{curly_bracket, is_opening_bracket_curly};
use crate::parser::match_brackets::find_bracket_end;
use crate::parser::parsers::block::BlockParser;
use crate::parser::tree_nodes::TreeNodes;
use crate::use_parser;
use crate::{
    parser::parse::Parser,
    parser::{grammar::GrammarLike, tree::TreeNode},
};

pub struct BlockGrammar {}

impl GrammarLike for BlockGrammar {
    use_parser!(BlockParser);

    fn next_match_start(&self, nodes: &TreeNodes) -> Option<usize> {
        for (index, node) in nodes.iter().enumerate() {
            if is_opening_bracket_curly(node) {
                return Some(index);
            }
        }

        None
    }

    fn next_match_end(&self, nodes: &TreeNodes, start_index: usize) -> Option<usize> {
        find_bracket_end(curly_bracket(), start_index, nodes.iter())
    }

    fn allow_parallel_processing(&self) -> bool {
        true
    }
}
