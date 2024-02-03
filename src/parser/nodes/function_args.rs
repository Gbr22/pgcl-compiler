use crate::{
    common::range::Range,
    parser::tree::{ParseError, TreeNode, TreeNodeLike},
};

#[derive(Debug, Clone)]
pub struct FunctionArgs {
    pub args: Vec<TreeNode>,
    pub range: Range,
}

impl TreeNodeLike for FunctionArgs {
    fn get_range(&self) -> Range {
        self.range
    }
    fn get_errors(&self) -> Vec<ParseError> {
        let errors: Vec<ParseError> = self.args.iter().flat_map(|arg| arg.get_errors()).collect();

        errors
    }
    fn children(&self) -> Vec<&TreeNode> {
        self.args.iter().collect()
    }
}
