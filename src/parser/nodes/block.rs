use crate::{
    common::range::Range,
    parser::tree::{ParseError, TreeNode, TreeNodeLike},
};

#[derive(Debug, Clone)]
pub struct Block {
    pub range: Range,
    pub children: Vec<TreeNode>,
}

impl TreeNodeLike for Block {
    fn get_range(&self) -> Range {
        self.range
    }
    fn get_errors(&self) -> Vec<ParseError> {
        let mut errors: Vec<ParseError> = vec![];
        for child in &self.children {
            errors.extend(child.get_errors());
        }

        errors
    }
}
