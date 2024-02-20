use crate::common::range::Range;
use crate::parser::nodes::block::PtBlock;
use crate::parser::nodes::var_declaration::PtVarDeclaration;
use crate::parser::tree::{TreeNode, TreeNodeLike};

use super::ret::ReturnStatement;
use super::simple::ExpressionStatement;

trait_enum! {
    #[derive(Debug, Clone)]
    pub enum Statement: StatementLike {
        ExpressionStatement,
        ReturnStatement
    }
}

pub trait StatementLike {
    fn to_node_like(&self) -> Box<&dyn TreeNodeLike>;
}

impl TreeNodeLike for Statement {
    fn get_range(&self) -> Range {
        self.to_node_like().get_range()
    }
    fn children(&self) -> Vec<&TreeNode> {
        self.to_node_like().children()
    }
}