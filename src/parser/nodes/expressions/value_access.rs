use crate::parser::tree::TreeNodeLike;

use super::expr::ExpressionLike;

#[derive(Debug, Clone)]
pub struct ValueAccess {
    name: String,
    start_index: usize,
    end_index: usize
}

impl TreeNodeLike for ValueAccess {
    fn get_start_index(&self) -> usize {
        self.start_index
    }

    fn get_end_index(&self) -> usize {
        self.end_index
    }
}

impl ExpressionLike for ValueAccess {
    fn to_node_like(&self) -> Box<&dyn TreeNodeLike> {
        Box::new(self)
    }
}