use crate::{
    common::range::Range,
    parser::tree::{ParseError, TreeNode, TreeNodeLike},
};

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    pub name: String,
    pub return_type: Box<TreeNode>,
    pub body: Box<TreeNode>,
    pub range: Range,
}

impl FunctionDeclaration {}

impl TreeNodeLike for FunctionDeclaration {
    fn get_range(&self) -> Range {
        self.range
    }
    fn get_errors(&self) -> Vec<ParseError> {
        let mut errors: Vec<ParseError> = vec![];
        errors.extend(self.return_type.get_errors());
        errors.extend(self.body.get_errors());

        errors
    }
    fn children(&self) -> Vec<&TreeNode> {
        vec![&self.return_type,&self.body]
    }
}
