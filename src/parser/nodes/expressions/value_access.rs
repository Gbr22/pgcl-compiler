use crate::{
    common::range::Range,
    parser::{
        program_tree::{
            program_tree::{CurrentContext, PtError, RootContext, RootContextMutRef, TryIntoPt},
            value_declaration::ValueDeclarationReferable,
        },
        reference::{Reference, ValueReference},
        tree::TreeNodeLike,
    },
};

use super::expr::ExpressionLike;

#[derive(Debug, Clone)]
pub struct ValueAccess {
    pub name: String,
    pub range: Range,
}

impl TreeNodeLike for ValueAccess {
    fn get_range(&self) -> Range {
        self.range
    }
    fn children(&self) -> Vec<&crate::parser::tree::TreeNode> {
        vec![]
    }
}

impl ExpressionLike for ValueAccess {
    fn to_node_like(&self) -> Box<&dyn TreeNodeLike> {
        Box::new(self)
    }
}

#[derive(Debug, Clone)]
pub struct PtValueAccess {
    pub name: String,
    pub range: Range,
    pub reference: ValueReference,
}

impl PtValueAccess {
    pub fn resolve(&self, root: &RootContext) -> Option<ValueDeclarationReferable> {
        self.reference.resolve(root)
    }
}

impl TryIntoPt<PtValueAccess> for ValueAccess {
    fn try_into_pt(
        self,
        _root_context: RootContextMutRef,
        context: &CurrentContext,
    ) -> Result<PtValueAccess, PtError> {
        let range = self.range;

        Ok(PtValueAccess {
            range,
            reference: ValueReference(Reference::new(
                &self.name,
                context.accessible_scopes.clone(),
            )),
            name: self.name,
        })
    }
}
