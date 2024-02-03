use crate::{
    common::range::Range,
    parser::tree::{TreeNode, TreeNodeLike},
};

#[derive(Debug, Clone)]
pub struct UniformDeclaration {
    pub name: String,
    pub typ: Box<TreeNode>,
    pub range: Range,
}

impl UniformDeclaration {}

impl TreeNodeLike for UniformDeclaration {
    fn get_range(&self) -> Range {
        self.range
    }
    fn children(&self) -> Vec<&TreeNode> {
        vec![&self.typ]
    }
}
