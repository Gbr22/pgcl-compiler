use crate::parser::tree::{TreeNodeLike, TreeNode, ParseError};
use super::value_access::ValueAccess;

trait_enum!{
    #[derive(Debug, Clone)]
    pub enum Expression: ExpressionLike {
        ValueAccess,
    }
}

pub trait ExpressionLike {
    fn to_node_like(&self) -> Box<&dyn TreeNodeLike>;
}

impl TreeNodeLike for Expression {
    fn get_start_index(&self) -> usize {
        self.to_node_like().get_start_index()
    }
    fn get_end_index(&self) -> usize {
        self.to_node_like().get_end_index()
    }
}

impl Expression {
    pub fn parse(nodes: Vec<TreeNode>) -> TreeNode {
        return ParseError::from_nodes(&nodes,format!("Test")).into();
    }
}