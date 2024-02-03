use crate::{
    common::range::Range,
    parser::tree::{TreeNode, TreeNodeLike},
};

#[derive(Debug, Clone)]
pub struct Document {
    pub range: Range,
    pub children: Vec<TreeNode>,
}

impl Document {}

impl TreeNodeLike for Document {
    fn get_range(&self) -> Range {
        self.range
    }
    fn children(&self) -> Vec<&TreeNode> {
        return self.children.iter().collect();
    }
}
