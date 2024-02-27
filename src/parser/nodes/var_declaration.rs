use crate::{
    common::range::Range,
    lexer::types::address_spaces::FUNCTION,
    parser::{
        program_tree::{
            program_tree::{CurrentContext, PtError, RootContextMutRef, TryIntoPt},
            scope::{Referable, ScopeId},
            value_declaration::ValueDeclarationReferableLike,
        },
        tree::{TreeNode, TreeNodeLike},
    },
};

use super::{tagged_string::TaggedString, types::typ::PtTypeExpression};

#[derive(Debug, Clone)]
pub struct AstVarDeclaration {
    pub name: String,
    pub typ: Box<TreeNode>,
    pub range: Range,
    pub address_space: Option<TaggedString>,
}

#[derive(Debug, Clone)]
pub struct PtVarDeclaration {
    pub range: Range,
    pub name: String,
    pub typ: PtTypeExpression,
    pub address_space: String,
    pub address_space_range: Option<Range>,
}

impl Referable for PtVarDeclaration {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl ValueDeclarationReferableLike for PtVarDeclaration {
    fn get_type(&self) -> PtTypeExpression {
        self.typ.clone()
    }
}

impl TryIntoPt<PtVarDeclaration> for AstVarDeclaration {
    fn try_into_pt(
        self,
        root_context: RootContextMutRef,
        context: &CurrentContext,
    ) -> Result<PtVarDeclaration, PtError> {
        let range = self.range;
        let name = self.name;
        let TreeNode::AstType(typ) = *self.typ else {
            return Err(PtError::in_at(&context.uri, self.range, "Expected type."));
        };

        let typ = typ.try_into_pt(root_context, context)?;

        let is_in_function = context.accessible_scopes.iter().any(|scope| {
            if let ScopeId::Function(_) = scope {
                true
            } else {
                false
            }
        });

        let (address_space_range, address_space) = match (self.address_space, is_in_function) {
            (Some(a), _) => (Some(a.range), a.value),
            (None, true) => (None, FUNCTION.to_owned()),
            (None, false) => {
                return Err(PtError::in_at(
                    &context.uri,
                    self.range,
                    "A variable declaration in this scope must define an address space."
                        .to_string(),
                ));
            }
        };

        Ok(PtVarDeclaration {
            range,
            name,
            typ,
            address_space,
            address_space_range,
        })
    }
}

impl TreeNodeLike for AstVarDeclaration {
    fn get_range(&self) -> Range {
        self.range
    }
    fn children(&self) -> Vec<&TreeNode> {
        vec![&self.typ]
    }
}
