use crate::common::range::Range;
use crate::parser::tree::{TreeNode, TreeNodeLike};

#[derive(Debug, Clone)]
pub struct TypeArgs {
    pub args: Vec<TreeNode>,
    pub range: Range,
}

impl TreeNodeLike for TypeArgs {
    fn get_range(&self) -> Range {
        self.range
    }
    fn children(&self) -> Vec<&TreeNode> {
        self.args.iter().collect()
    }
}
