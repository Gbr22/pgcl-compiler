use super::simple::SimpleType;
use crate::{
    parser::{
        grammars::types::simple::SimpleTypeGrammar,
        tree::{ParseError, TreeNode, TreeNodeLike},
        tree_nodes::TreeNodes,
    },
    process_grammars,
};

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
            return ParseError::at(
                nodes.range,
                format!(
                    "Could not combine types. Multiple types detected where only one is expected."
                ),
            )
            .into();
        }
        if nodes.len() == 0 {
            return ParseError::at(
                nodes.range,
                format!("Could not parse type. Expected type, found nothing."),
            )
            .into();
        }
        let node = nodes.into_first().unwrap();

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
