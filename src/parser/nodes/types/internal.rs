use crate::{common::range::Range, parser::{program_tree::{program_tree::RootContext, scope::ScopeId, type_declaration::{TypeDeclarationReferable, TypeDeclarationReferableLike}}, reference::{Reference, TypeReference}}};

use super::typ::{PtType, PtTypeLike};


#[derive(Debug, Clone)]
pub struct PtInternalTypeExpression {
    pub reference: TypeReference,
}

impl PtTypeLike for PtInternalTypeExpression {
    fn to_string(&self, root: &RootContext) -> String {
        let Some(referable) = self.reference.resolve(root) else {
            return format!("error<\"Could not resolve reference to type\">");
        };
        referable.to_string()
    }
    fn resolve_type(&self, root: &RootContext) -> Option<TypeDeclarationReferable> {
        self.reference.resolve(root)
    }
}

impl Into<PtType> for PtInternalTypeExpression {
    fn into(self) -> PtType {
        PtType::Internal(self)
    }
}

pub fn global_type_ref(name: impl Into<String>) -> PtInternalTypeExpression {
    PtInternalTypeExpression {
        reference: TypeReference(
            Reference {
                scopes: vec![
                    ScopeId::Global
                ],
                name: name.into()
            }
        )
    }
}