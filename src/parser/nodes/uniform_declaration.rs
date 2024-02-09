use std::sync::{Arc, Mutex};

use crate::{
    common::range::Range,
    parser::{
        program_tree::{program_tree::{CurrentContext, PtError, RootContext, TryIntoPt}, scope::Referable, value_declaration::ValueDeclarationReferableLike},
        tree::{TreeNode, TreeNodeLike},
    },
};

use super::types::typ::PtType;

#[derive(Debug, Clone)]
pub struct AstUniformDeclaration {
    pub name: String,
    pub typ: Box<TreeNode>,
    pub range: Range,
}

#[derive(Debug,Clone)]
pub struct PtUniformDeclaration {
    pub range: Range,
    pub name: String,
    pub typ: PtType,
}

impl Referable for PtUniformDeclaration {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl ValueDeclarationReferableLike for PtUniformDeclaration {}

impl TryIntoPt<PtUniformDeclaration> for AstUniformDeclaration {
    fn try_into_pt(
        self,
        root_context: Arc<Mutex<RootContext>>,
        context: &CurrentContext,
    ) -> Result<PtUniformDeclaration, PtError> {
        let range = self.range;
        let name = self.name;
        let TreeNode::AstType(typ) = *self.typ else {
            return Err(PtError {
                range: Some(self.range),
                message: format!("Expected type."),
            });
        };
        
        let typ = typ.try_into_pt(root_context, context)?;

        Ok(PtUniformDeclaration { range, name, typ })
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
