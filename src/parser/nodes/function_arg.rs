use std::sync::{Arc, Mutex};

use crate::common::range::Range;
use crate::parser::program_tree::program_tree::{CurrentContext, PtError, RootContext, TryIntoPt};
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

impl TryIntoPt<PtFunctionArg> for AstFunctionArg {
    fn try_into_pt(
        self,
        root_context: Arc<Mutex<RootContext>>,
        context: &CurrentContext,
    ) -> Result<PtFunctionArg, PtError> {
        let range = self.range;
        let name = self.name;
        let TreeNode::AstType(typ) = *self.typ else {
            return Err(PtError {
                range: Some(self.range),
                message: format!("Expected type."),
            });
        };
        
        let typ = typ.try_into_pt(root_context, context)?;

        Ok(PtFunctionArg { range, name, typ })
    }
}