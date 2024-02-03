use crate::parser::{
    grammar::GrammarLike, nodes::document::Document, tree::TreeNode, tree_nodes::TreeNodes,
};

pub struct DocumentGrammar {}

impl GrammarLike for DocumentGrammar {
    fn next_match_start(&self, nodes: &TreeNodes) -> Option<usize> {
        if nodes.len() == 0 {
            None
        } else {
            Some(0) // match at first node
        }
    }
    fn next_match_end(&self, nodes: &TreeNodes, _start_index: usize) -> Option<usize> {
        Some(nodes.len() - 1)
    }
    fn construct(&self, nodes: TreeNodes) -> TreeNode {
        let document = Document::parse(nodes);

        document
    }
}
