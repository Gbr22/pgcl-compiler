use crate::{
    common::range::Range,
    parser::tree::{TreeNode, TreeNodeLike},
};

#[derive(Debug, Clone)]
pub struct FunctionArgs {
    pub args: Vec<TreeNode>,
    pub range: Range,
}

impl TreeNodeLike for FunctionArgs {
    fn get_range(&self) -> Range {
        self.range
    }
    fn children(&self) -> Vec<&TreeNode> {
        self.args.iter().collect()
    }
}
