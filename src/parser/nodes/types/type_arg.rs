use crate::common::range::Range;
use crate::parser::tree::{ParseError, TreeNode, TreeNodeLike};

#[derive(Debug, Clone)]
pub struct TypeArg {
    pub value: Box<TreeNode>,
    pub range: Range,
}

impl TreeNodeLike for TypeArg {
    fn get_range(&self) -> Range {
        self.range
    }
    fn get_errors(&self) -> Vec<ParseError> {
        self.value.get_errors()
    }
    fn children(&self) -> Vec<&TreeNode> {
        vec![&self.value]
    }
}
