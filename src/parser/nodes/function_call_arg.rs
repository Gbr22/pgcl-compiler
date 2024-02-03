use crate::common::range::Range;
use crate::parser::tree::{ParseError, TreeNode, TreeNodeLike};

#[derive(Debug, Clone)]
pub struct FunctionCallArg {
    pub expr: Box<TreeNode>,
    pub range: Range,
}

impl TreeNodeLike for FunctionCallArg {
    fn get_range(&self) -> Range {
        self.range
    }
    fn get_errors(&self) -> Vec<ParseError> {
        self.expr.get_errors()
    }
    fn children(&self) -> Vec<&TreeNode> {
        vec![&self.expr]
    }
}
