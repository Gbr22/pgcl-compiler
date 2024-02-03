use crate::{
    common::range::Range,
    parser::tree::{TreeNode, TreeNodeLike},
};

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    pub range: Range,
    pub name: String,
    pub args: Box<TreeNode>,
    pub return_type: Box<TreeNode>,
    pub body: Box<TreeNode>,
}

impl FunctionDeclaration {}

impl TreeNodeLike for FunctionDeclaration {
    fn get_range(&self) -> Range {
        self.range
    }
    fn children(&self) -> Vec<&TreeNode> {
        vec![&self.args, &self.return_type, &self.body]
    }
}
