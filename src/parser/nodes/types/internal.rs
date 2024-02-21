use crate::parser::{
    program_tree::{
        program_tree::RootContext,
        scope::ScopeId,
        type_declaration::{TypeDeclarationReferable, TypeDeclarationReferableLike},
    },
    reference::{Reference, TypeReference},
};

use super::typ::{PtType, PtTypeLike};

#[derive(Debug, Clone)]
pub struct PtInternalTypeExpression {
    pub reference: TypeReference,
}

impl PtTypeLike for PtInternalTypeExpression {
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

impl From<PtInternalTypeExpression> for PtType {
    fn from(val: PtInternalTypeExpression) -> Self {
        PtType::Internal(val)
    }
}

pub fn global_type_ref(name: impl Into<String>) -> PtInternalTypeExpression {
    PtInternalTypeExpression {
        reference: TypeReference(Reference {
            scopes: vec![ScopeId::Global],
            name: name.into(),
        }),
    }
}
