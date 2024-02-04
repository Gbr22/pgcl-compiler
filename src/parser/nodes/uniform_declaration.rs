use crate::{
    common::range::Range,
    parser::{
        program_tree::PtError,
        tree::{TreeNode, TreeNodeLike},
    },
};

#[derive(Debug, Clone)]
pub struct AstUniformDeclaration {
    pub name: String,
    pub typ: Box<TreeNode>,
    pub range: Range,
}

#[derive(Debug)]
pub struct PtUniformDeclaration {
    pub range: Range,
    pub name: String,
}

impl TryFrom<AstUniformDeclaration> for PtUniformDeclaration {
    type Error = PtError;

    fn try_from(value: AstUniformDeclaration) -> Result<Self, Self::Error> {
        let range = value.range;
        let name = value.name;

        Ok(PtUniformDeclaration { range, name })
    }
}

impl TreeNodeLike for AstUniformDeclaration {
    fn get_range(&self) -> Range {
        self.range
    }
    fn children(&self) -> Vec<&TreeNode> {
        vec![&self.typ]
    }
}
