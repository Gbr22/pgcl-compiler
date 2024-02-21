use super::compound::CompoundType;
use super::internal::PtInternalTypeExpression;
use super::simple::{AstSimpleTypeExpression, PtSimpleTypeExpression};
use crate::common::range::Range;
use crate::parser::program_tree::program_tree::{
    PtError, RootContext, RootContextMutRef, TryIntoPt,
};
use crate::parser::program_tree::type_declaration::TypeDeclarationReferable;
use crate::parser::tree::TreeNodeLike;

#[derive(Debug, Clone)]
pub enum AstType {
    Simple(AstSimpleTypeExpression),
    Compound(CompoundType),
}

impl AstTypeLike for AstType {
    fn to_node_like(&self) -> Box<&dyn TreeNodeLike> {
        match self {
            AstType::Simple(e) => e.to_node_like(),
            AstType::Compound(e) => e.to_node_like(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PtType {
    Simple(PtSimpleTypeExpression),
    Internal(PtInternalTypeExpression),
}

impl TryIntoPt<PtType> for AstType {
    fn try_into_pt(
        self,
        root_context: RootContextMutRef,
        context: &crate::parser::program_tree::program_tree::CurrentContext,
    ) -> Result<PtType, crate::parser::program_tree::program_tree::PtError> {
        match self {
            AstType::Simple(st) => {
                let simple_type = st.try_into_pt(root_context, context)?;

                Ok(PtType::Simple(simple_type))
            }
            _ => Err(PtError::in_at(
                &context.uri,
                self.get_range(),
                "Unknown type",
            )),
        }
    }
}

pub trait PtTypeLike {
    fn to_string(&self, root: &RootContext) -> String;
    fn resolve_type(&self, root: &RootContext) -> Option<TypeDeclarationReferable>;
}

impl PtTypeLike for PtType {
    fn to_string(&self, root: &RootContext) -> String {
        match self {
            PtType::Simple(e) => e.to_string(root),
            PtType::Internal(e) => e.to_string(root),
        }
    }

    fn resolve_type(&self, root: &RootContext) -> Option<TypeDeclarationReferable> {
        match self {
            PtType::Simple(e) => e.resolve_type(root),
            PtType::Internal(e) => e.resolve_type(root),
        }
    }
}

pub trait AstTypeLike {
    fn to_node_like(&self) -> Box<&dyn TreeNodeLike>;
}

impl TreeNodeLike for AstType {
    fn get_range(&self) -> Range {
        self.to_node_like().get_range()
    }
    fn children(&self) -> Vec<&crate::parser::tree::TreeNode> {
        self.to_node_like().children()
    }
}
