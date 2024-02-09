use super::typ::{AstTypeLike, PtTypeLike};
use crate::common::range::Range;

use crate::parser::program_tree::program_tree::TryIntoPt;
use crate::parser::program_tree::type_declaration::TypeDeclarationReferableLike;
use crate::parser::reference::{Reference, TypeReference};
use crate::parser::tree::TreeNodeLike;

#[derive(Debug, Clone)]
pub struct AstSimpleType {
    pub range: Range,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct PtSimpleType {
    pub reference: TypeReference,
    pub range: Range,
}

impl PtTypeLike for PtSimpleType {
    fn get_range(&self) -> Range {
        self.range
    }
    fn to_string(&self) -> String {
        let Some(referable) = self.reference.resolve() else {
            return format!("error<\"Could not resolve reference to type\">");
        };
        referable.to_string()
    }
}

impl TryIntoPt<PtSimpleType> for AstSimpleType {
    fn try_into_pt(
        self,
        root_context: std::sync::Arc<std::sync::Mutex<crate::parser::program_tree::program_tree::RootContext>>,
        context: &crate::parser::program_tree::program_tree::CurrentContext,
    ) -> Result<PtSimpleType, crate::parser::program_tree::program_tree::PtError> {
        Ok(PtSimpleType {
            reference: TypeReference(Reference {
                root: root_context.clone(),
                scopes: context.accessible_scopes.clone(),
                name: self.name
            }),
            range: self.range,
        })
    }
}

impl TreeNodeLike for AstSimpleType {
    fn get_range(&self) -> Range {
        self.range
    }
    fn children(&self) -> Vec<&crate::parser::tree::TreeNode> {
        vec![]
    }
}

impl AstTypeLike for AstSimpleType {
    fn to_node_like(&self) -> Box<&dyn TreeNodeLike> {
        Box::new(self)
    }
}
