use std::sync::{Arc, Mutex};

use crate::common::range::Range;
use crate::parser::program_tree::program_tree::{CurrentContext, PtError, RootContext, RootContextMutRef, RootContextMutRefType, TryIntoPt};
use crate::parser::program_tree::scope::Referable;
use crate::parser::program_tree::value_declaration::{ValueDeclarationReferable, ValueDeclarationReferableLike};
use crate::parser::tree::{TreeNode, TreeNodeLike};

use super::function_declaration::{AstFunctionDeclaration, PtFunctionDeclaration};
use super::types::typ::PtType;

#[derive(Debug, Clone)]
pub struct AstFunctionArg {
    pub name: String,
    pub typ: Box<TreeNode>,
    pub range: Range,
}

impl TreeNodeLike for AstFunctionArg {
    fn get_range(&self) -> Range {
        self.range
    }
    fn children(&self) -> Vec<&TreeNode> {
        vec![&self.typ]
    }
}

#[derive(Debug, Clone)]
pub struct PtFunctionArg {
    pub range: Range,
    pub name: String,
    pub typ: PtType
}

impl ValueDeclarationReferableLike for PtFunctionArg {
    fn get_type(&self) -> PtType {
        self.typ.clone()
    }
}

impl Referable for PtFunctionArg {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl TryIntoPt<PtFunctionArg> for AstFunctionArg {
    fn try_into_pt(
        self,
        root_context: RootContextMutRef,
        context: &CurrentContext,
    ) -> Result<PtFunctionArg, PtError> {
        let range = self.range;
        let name = self.name;
        let TreeNode::AstType(typ) = *self.typ else {
            return Err(PtError::in_at(&context.uri, self.range, "Expected type."));
        };
        
        let typ = typ.try_into_pt(root_context, context)?;

        Ok(PtFunctionArg { range, name, typ })
    }
}