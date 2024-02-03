use crate::{
    common::range::Range,
    parser::tree::{TreeNode, TreeNodeLike},
};

use super::statement::StatementLike;

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub range: Range,
    pub expr: Box<TreeNode>,
}

impl ReturnStatement {}

impl TreeNodeLike for ReturnStatement {
    fn get_range(&self) -> Range {
        self.range
    }
    fn children(&self) -> Vec<&TreeNode> {
        vec![&self.expr]
    }
}

impl StatementLike for ReturnStatement {
    fn to_node_like(&self) -> Box<&dyn TreeNodeLike> {
        Box::new(self)
    }
}
