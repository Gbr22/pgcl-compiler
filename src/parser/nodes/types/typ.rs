
use crate::{parser::{tree::{TreeNode, TreeNodeLike, ParseError}, grammars::types::simple::SimpleTypeGrammar, tree_nodes::TreeNodes}, process_grammars};
use super::simple::SimpleType;

trait_enum! {
    #[derive(Debug, Clone)]
    pub enum Type: TypeLike {
        SimpleType
    }
}

impl Type {
    pub fn parse(nodes: TreeNodes) -> TreeNode {
        let nodes = process_grammars! { nodes [
            SimpleTypeGrammar
        ] };

        if nodes.len() > 1 {
            return ParseError::from_nodes(&nodes.vec, format!("Could not combine types. Multiple types detected where only one is expected.")).into();
        }
        if nodes.len() == 0 {
            return ParseError::from_nodes(&nodes.vec, format!("Could not parse type. Expected type, found nothing.")).into();
        }
        let node = nodes.vec[0].clone();
        
        node
    }
}

pub trait TypeLike {
    fn to_node_like(&self) -> Box<&dyn TreeNodeLike>;
}

impl TreeNodeLike for Type {
    fn get_range(&self) -> crate::common::range::Range {
        self.to_node_like().get_range()
    }
    fn get_errors(&self) -> Vec<ParseError> {
        self.to_node_like().get_errors()
    }
}