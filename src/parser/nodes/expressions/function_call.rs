use crate::{
    common::range::Range,
    parser::tree::{TreeNode, TreeNodeLike},
};

use super::expr::ExpressionLike;

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub range: Range,
    pub args: Box<TreeNode>,
}

impl TreeNodeLike for FunctionCall {
    fn get_range(&self) -> Range {
        self.range
    }

    fn children(&self) -> Vec<&TreeNode> {
        vec![&self.args]
    }
}

impl ExpressionLike for FunctionCall {
    fn to_node_like(&self) -> Box<&dyn TreeNodeLike> {
        Box::new(self)
    }
}
