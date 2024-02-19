use super::statement::StatementLike;
use crate::{
    common::range::Range,
    parser::tree::{TreeNode, TreeNodeLike},
};

// Semicolon delimited statement
#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub range: Range,
    pub expr: Box<TreeNode>,
}

impl TreeNodeLike for ExpressionStatement {
    fn get_range(&self) -> Range {
        self.range
    }
    fn children(&self) -> Vec<&TreeNode> {
        vec![&self.expr]
    }
}

impl StatementLike for ExpressionStatement {
    fn to_node_like(&self) -> Box<&dyn TreeNodeLike> {
        Box::new(self)
    }
}
