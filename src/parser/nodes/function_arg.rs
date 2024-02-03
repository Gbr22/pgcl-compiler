use crate::common::range::Range;
use crate::parser::tree::{ParseError, TreeNode, TreeNodeLike};

#[derive(Debug, Clone)]
pub struct FunctionArg {
    pub name: String,
    pub typ: Box<TreeNode>,
    pub range: Range,
}

impl TreeNodeLike for FunctionArg {
    fn get_range(&self) -> Range {
        self.range
    }
    fn get_errors(&self) -> Vec<ParseError> {
        self.typ.get_errors()
    }
    fn children(&self) -> Vec<&TreeNode> {
        vec![&self.typ]
    }
}
