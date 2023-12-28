use super::{tree::TreeNode, document::Document, uniform::UniformGrammar};

trait_enum!{
    pub enum Grammar: GrammarLike {
        DocumentGrammar,
        UniformGrammar
    }
}


pub trait GrammarLike {
    fn next_match_at(&self, _nodes: &[TreeNode]) -> Option<usize>;
    fn find_match_end(&self, nodes: &[TreeNode], start_index: usize) -> Option<usize>;
    fn has_match(&self, nodes: &[TreeNode]) -> bool {
        self.next_match_at(nodes).is_some()
    }
    fn construct(&self, nodes: Vec<TreeNode>) -> TreeNode;
    fn take_next(&self, mut nodes: Vec<TreeNode>) -> (Option<TreeNode>, Vec<TreeNode>) {
        let start_index = self.next_match_at(&nodes);
        let Some(start_index) = start_index else {
            return (None, nodes);
        };
        let end_index = self.find_match_end(&nodes, start_index);
        let Some(end_index) = end_index else {
            return (None, nodes);
        };

        let inner_nodes = nodes
            .splice(start_index..(end_index+1),vec![])
            .collect();

        let new_node = self.construct(inner_nodes);

        (Some(new_node),nodes)
    }
}

pub struct DocumentGrammar {}

impl GrammarLike for DocumentGrammar {
    fn next_match_at(&self, nodes: &[TreeNode]) -> Option<usize> {
        if nodes.len() == 0 {
            None
        } else {
            Some(0) // match at first node
        }
    }
    fn find_match_end(&self, nodes: &[TreeNode], start_index: usize) -> Option<usize> {
        Some(nodes.len()-1)
    }
    fn construct(&self, nodes: Vec<TreeNode>) -> TreeNode {
        let document = Document::parse(nodes);

        document
    }
}