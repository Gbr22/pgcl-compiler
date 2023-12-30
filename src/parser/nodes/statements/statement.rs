use crate::parser::tree::{TreeNode, TreeNodeLike, ParseError};

use super::return_statement::ReturnStatement;
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
    fn get_start_index(&self) -> usize {
        self.to_node_like().get_start_index()
    }

    fn get_end_index(&self) -> usize {
        self.to_node_like().get_end_index()
    }

    fn get_errors(&self) -> Vec<ParseError> {
        self.to_node_like().get_errors()
    }
}