use crate::{
    common::range::Range,
    parser::tree::{TreeNode, TreeNodeLike},
};

#[derive(Debug, Clone)]
pub struct Block {
    pub range: Range,
    pub children: Vec<TreeNode>,
}

impl TreeNodeLike for Block {
    fn get_range(&self) -> Range {
        self.range
    }
    fn children(&self) -> Vec<&TreeNode> {
        return self.children.iter().collect();
    }
}
