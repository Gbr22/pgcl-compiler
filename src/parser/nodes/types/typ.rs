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
