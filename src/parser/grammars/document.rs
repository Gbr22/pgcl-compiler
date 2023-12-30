use crate::parser::{grammar::GrammarLike, tree::TreeNode, nodes::document::Document};

pub struct DocumentGrammar {}

impl GrammarLike for DocumentGrammar {
    fn next_match_start(&self, nodes: &[TreeNode]) -> Option<usize> {
        if nodes.len() == 0 {
            None
        } else {
            Some(0) // match at first node
        }
    }
    fn next_match_end(&self, nodes: &[TreeNode], _start_index: usize) -> Option<usize> {
        Some(nodes.len()-1)
    }
    fn construct(&self, nodes: Vec<TreeNode>) -> TreeNode {
        let document = Document::parse(nodes);

        document
    }
}