use crate::{
    common::range::Range,
    parser::tree::{ParseError, TreeNode, TreeNodeLike},
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

    fn get_errors(&self) -> Vec<ParseError> {
        self.args.get_errors()
    }
}

impl ExpressionLike for FunctionCall {
    fn to_node_like(&self) -> Box<&dyn TreeNodeLike> {
        Box::new(self)
    }
}
