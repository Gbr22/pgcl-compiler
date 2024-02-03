use crate::common::range::Range;
use crate::parser::tree::{ParseError, TreeNode, TreeNodeLike};

#[derive(Debug, Clone)]
pub struct TypeArgs {
    pub args: Vec<TreeNode>,
    pub range: Range,
}

impl TreeNodeLike for TypeArgs {
    fn get_range(&self) -> Range {
        self.range
    }
    fn get_errors(&self) -> Vec<ParseError> {
        let mut errors: Vec<ParseError> = vec![];
        for child in &self.args {
            errors.extend(child.get_errors());
        }

        errors
    }
    fn children(&self) -> Vec<&TreeNode> {
        self.args.iter().collect()
    }
}
