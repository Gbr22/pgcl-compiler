use crate::common::range::Range;
use crate::parser::tree::{TreeNode, TreeNodeLike};

#[derive(Debug, Clone)]
pub struct FunctionCallArg {
    pub expr: Box<TreeNode>,
    pub range: Range,
}

impl TreeNodeLike for FunctionCallArg {
    fn get_range(&self) -> Range {
        self.range
    }
    fn children(&self) -> Vec<&TreeNode> {
        vec![&self.expr]
    }
}
