use std::sync::{Arc, Mutex};

use crate::{
    common::range::Range,
    parser::{
        program_tree::{
            function_declaration::FunctionDeclarationReferableLike, program_tree::{CurrentContext, PtError, RootContext, TryIntoPt}, scope::Referable, value_declaration::ValueDeclarationReferableLike
        },
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

impl TryIntoPt<PtFunctionDeclaration> for AstFunctionDeclaration {
    fn try_into_pt(
        self,
        root_context: Arc<Mutex<RootContext>>,
        context: &CurrentContext,
    ) -> Result<PtFunctionDeclaration, PtError> {
        let range = self.range;
        let name = self.name;

        Ok(PtFunctionDeclaration { range, name })
    }
}

#[derive(Debug, Clone)]
pub struct PtFunctionDeclaration {
    pub range: Range,
    pub name: String,
}

impl Referable for PtFunctionDeclaration {
    fn get_name(&self) -> &str {
        &self.name
    }
}
impl FunctionDeclarationReferableLike for PtFunctionDeclaration {}

impl TreeNodeLike for AstFunctionDeclaration {
    fn get_range(&self) -> Range {
        self.range
    }
    fn children(&self) -> Vec<&TreeNode> {
        vec![&self.args, &self.return_type, &self.body]
    }
}
