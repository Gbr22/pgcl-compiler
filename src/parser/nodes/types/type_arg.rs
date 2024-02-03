use crate::common::range::Range;
use crate::parser::tree::{TreeNode, TreeNodeLike};

#[derive(Debug, Clone)]
pub struct TypeArg {
    pub value: Box<TreeNode>,
    pub range: Range,
}

impl TreeNodeLike for TypeArg {
    fn get_range(&self) -> Range {
        self.range
    }
    fn children(&self) -> Vec<&TreeNode> {
        vec![&self.value]
    }
}
