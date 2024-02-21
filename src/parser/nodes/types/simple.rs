use super::typ::{AstTypeLike, PtTypeLike};
use crate::common::range::Range;

use crate::parser::program_tree::program_tree::{RootContext, RootContextMutRef, TryIntoPt};
use crate::parser::program_tree::type_declaration::{
    TypeDeclarationReferable, TypeDeclarationReferableLike,
};
use crate::parser::reference::{Reference, TypeReference};
use crate::parser::tree::TreeNodeLike;

#[derive(Debug, Clone)]
pub struct AstSimpleTypeExpression {
    pub range: Range,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct PtSimpleTypeExpression {
    pub reference: TypeReference,
    pub range: Range,
}

impl PtTypeLike for PtSimpleTypeExpression {
    fn to_string(&self, root: &RootContext) -> String {
        let Some(referable) = self.reference.resolve(root) else {
            return "error<\"Could not resolve reference to type\">".to_string();
        };
        referable.to_string()
    }
    fn resolve_type(&self, root: &RootContext) -> Option<TypeDeclarationReferable> {
        self.reference.resolve(root)
    }
}

impl TryIntoPt<PtSimpleTypeExpression> for AstSimpleTypeExpression {
    fn try_into_pt(
        self,
        _root_context: RootContextMutRef,
        context: &crate::parser::program_tree::program_tree::CurrentContext,
    ) -> Result<PtSimpleTypeExpression, crate::parser::program_tree::program_tree::PtError> {
        Ok(PtSimpleTypeExpression {
            reference: TypeReference(Reference {
                scopes: context.accessible_scopes.clone(),
                name: self.name,
            }),
            range: self.range,
        })
    }
}

impl TreeNodeLike for AstSimpleTypeExpression {
    fn get_range(&self) -> Range {
        self.range
    }
    fn children(&self) -> Vec<&crate::parser::tree::TreeNode> {
        vec![]
    }
}

impl AstTypeLike for AstSimpleTypeExpression {
    fn to_node_like(&self) -> Box<&dyn TreeNodeLike> {
        Box::new(self)
    }
}
