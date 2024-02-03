use super::statement::StatementLike;
use crate::{
    common::range::Range,
    parser::tree::{ParseError, TreeNode, TreeNodeLike},
};

// Semicolon delimited statement
#[derive(Debug, Clone)]
pub struct SimpleStatement {
    pub range: Range,
    pub expr: Box<TreeNode>,
}

impl TreeNodeLike for SimpleStatement {
    fn get_range(&self) -> Range {
        self.range
    }
    fn get_errors(&self) -> Vec<ParseError> {
        self.expr.get_errors()
    }
}

impl StatementLike for SimpleStatement {
    fn to_node_like(&self) -> Box<&dyn TreeNodeLike> {
        Box::new(self)
    }
}
