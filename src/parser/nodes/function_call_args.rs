use crate::{
    common::range::Range,
    parser::tree::{TreeNode, TreeNodeLike},
};

#[derive(Debug, Clone)]
pub struct FunctionCallArgs {
    pub args: Vec<TreeNode>,
    pub range: Range,
}

impl TreeNodeLike for FunctionCallArgs {
    fn get_range(&self) -> Range {
        self.range
    }
    fn children(&self) -> Vec<&TreeNode> {
        self.args.iter().collect()
    }
}
