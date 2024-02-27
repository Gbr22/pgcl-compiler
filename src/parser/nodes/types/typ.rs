use super::compound::CompoundType;
use super::internal::PtInternalTypeExpression;
use super::simple::{AstSimpleTypeExpression, PtSimpleTypeExpression};
use crate::common::range::Range;
use crate::parser::program_tree::program_tree::{
    PtError, RootContext, RootContextMutRef, TryIntoPt,
};
use crate::parser::program_tree::type_declaration::TypeDeclarationReferable;
use crate::parser::tree::TreeNodeLike;
use enum_dispatch::enum_dispatch;

#[derive(Debug, Clone)]
#[enum_dispatch]
pub enum AstType {
    Simple(AstSimpleTypeExpression),
    Compound(CompoundType),
}

#[derive(Debug, Clone)]
#[enum_dispatch]
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

#[enum_dispatch(PtType)]
pub trait PtTypeLike {
    fn to_string(&self, root: &RootContext) -> String;
    fn resolve_type(&self, root: &RootContext) -> Option<TypeDeclarationReferable>;
}

#[enum_dispatch(AstType)]
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
