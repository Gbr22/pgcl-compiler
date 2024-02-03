use super::typ::TypeLike;
use crate::common::range::Range;

use crate::parser::tree::TreeNodeLike;

#[derive(Debug, Clone)]
pub struct SimpleType {
    pub range: Range,
    pub name: String,
}

impl SimpleType {}

impl TreeNodeLike for SimpleType {
    fn get_range(&self) -> Range {
        self.range
    }
}

impl TypeLike for SimpleType {
    fn to_node_like(&self) -> Box<&dyn TreeNodeLike> {
        Box::new(self)
    }
}
