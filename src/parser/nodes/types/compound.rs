use super::typ::TypeLike;
use crate::common::range::Range;

use crate::parser::tree::{TreeNode, TreeNodeLike};

#[derive(Debug, Clone)]
pub struct CompoundType {
    pub range: Range,
    pub name: String,
    pub args: Box<TreeNode>,
}

impl CompoundType {}

impl TreeNodeLike for CompoundType {
    fn get_range(&self) -> Range {
        self.range
    }
    fn children(&self) -> Vec<&TreeNode> {
        self.args.iter().collect()
    }
}

impl TypeLike for CompoundType {
    fn to_node_like(&self) -> Box<&dyn TreeNodeLike> {
        Box::new(self)
    }
}
