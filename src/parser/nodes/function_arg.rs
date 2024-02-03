use crate::common::range::Range;
use crate::parser::tree::{TreeNode, TreeNodeLike};

#[derive(Debug, Clone)]
pub struct FunctionArg {
    pub name: String,
    pub typ: Box<TreeNode>,
    pub range: Range,
}

impl TreeNodeLike for FunctionArg {
    fn get_range(&self) -> Range {
        self.range
    }
    fn children(&self) -> Vec<&TreeNode> {
        vec![&self.typ]
    }
}
