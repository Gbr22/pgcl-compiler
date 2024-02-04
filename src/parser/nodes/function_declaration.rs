use crate::{
    common::range::Range,
    parser::{
        program_tree::PtError,
        tree::{TreeNode, TreeNodeLike},
    },
};

#[derive(Debug, Clone)]
pub struct AstFunctionDeclaration {
    pub range: Range,
    pub name: String,
    pub args: Box<TreeNode>,
    pub return_type: Box<TreeNode>,
    pub body: Box<TreeNode>,
}

impl TryFrom<AstFunctionDeclaration> for PtFunctionDeclaration {
    type Error = PtError;

    fn try_from(value: AstFunctionDeclaration) -> Result<Self, Self::Error> {
        let range = value.range;
        let name = value.name;

        Ok(PtFunctionDeclaration { range, name })
    }
}

#[derive(Debug)]
pub struct PtFunctionDeclaration {
    pub range: Range,
    pub name: String,
}

impl TreeNodeLike for AstFunctionDeclaration {
    fn get_range(&self) -> Range {
        self.range
    }
    fn children(&self) -> Vec<&TreeNode> {
        vec![&self.args, &self.return_type, &self.body]
    }
}
