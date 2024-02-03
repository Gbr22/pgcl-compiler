use crate::{common::range::Range, parser::tree::TreeNodeLike};

use super::expr::ExpressionLike;

#[derive(Debug, Clone)]
pub struct ValueAccess {
    pub name: String,
    pub range: Range,
}

impl TreeNodeLike for ValueAccess {
    fn get_range(&self) -> Range {
        self.range
    }
    fn children(&self) -> Vec<&crate::parser::tree::TreeNode> {
        vec![]
    }
}

impl ExpressionLike for ValueAccess {
    fn to_node_like(&self) -> Box<&dyn TreeNodeLike> {
        Box::new(self)
    }
}
