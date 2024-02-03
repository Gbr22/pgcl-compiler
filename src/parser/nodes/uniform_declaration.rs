use crate::{
    common::range::Range,
    parser::tree::{ParseError, TreeNode, TreeNodeLike},
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
    fn get_errors(&self) -> Vec<ParseError> {
        let typ: TreeNode = *self.typ.clone();
        let TreeNode::ParseError(error) = typ else {
            return vec![];
        };

        vec![error]
    }
}
