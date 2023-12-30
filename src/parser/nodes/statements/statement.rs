use crate::parser::tree::{TreeNode, TreeNodeLike, ParseError};

use super::ret::ReturnStatement;
use super::simple_statement::SimpleStatement;

trait_enum! {
    #[derive(Debug, Clone)]
    pub enum Statement: StatementLike {
        SimpleStatement,
        ReturnStatement
    }
}

pub trait StatementLike {
    fn to_node_like(&self) -> Box<&dyn TreeNodeLike>;
}

impl TreeNodeLike for Statement {
    fn get_range(&self) -> crate::common::range::Range {
        self.to_node_like().get_range()
    }

    fn get_errors(&self) -> Vec<ParseError> {
        self.to_node_like().get_errors()
    }
}