use super::compound::CompoundType;
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
pub enum PtConcreteTypeExpression {
    Simple(PtSimpleTypeExpression),
    //TODO: Compound(PtCompoundTypeExpression)
}

impl PtTypeExpressionLike for PtConcreteTypeExpression {
    fn to_string(&self, root: &RootContext) -> String {
        match self {
            PtConcreteTypeExpression::Simple(s) => s.to_string(root),
        }
    }
    fn get_description(&self, root: &RootContext) -> Option<String> {
        match self {
            PtConcreteTypeExpression::Simple(s) => s.get_description(root),
        }
    }
}

#[derive(Debug, Clone)]
#[enum_dispatch]
pub enum PtTypeExpression {
    Conrete(PtConcreteTypeExpression),
    Union(UnionTypeExpression),
}

#[derive(Debug, Clone)]
pub struct UnionTypeExpression {
    pub items: Vec<PtTypeExpression>,
}

impl PtTypeExpressionLike for UnionTypeExpression {
    fn to_string(&self, root: &RootContext) -> String {
        format!("TODO | TODO")
    }
    fn get_description(&self, root: &RootContext) -> Option<String> {
        None
    }
}

impl TryIntoPt<PtTypeExpression> for AstType {
    fn try_into_pt(
        self,
        root_context: RootContextMutRef,
        context: &crate::parser::program_tree::program_tree::CurrentContext,
    ) -> Result<PtTypeExpression, crate::parser::program_tree::program_tree::PtError> {
        match self {
            AstType::Simple(st) => {
                let simple_type = st.try_into_pt(root_context, context)?;

                Ok(PtTypeExpression::Conrete(PtConcreteTypeExpression::Simple(
                    simple_type,
                )))
            }
            _ => Err(PtError::in_at(
                &context.uri,
                self.get_range(),
                "Unknown type",
            )),
        }
    }
}

#[enum_dispatch(PtTypeExpression)]
pub trait PtTypeExpressionLike {
    fn to_string(&self, root: &RootContext) -> String;
    fn get_description(&self, root: &RootContext) -> Option<String>;
}

#[enum_dispatch(PtConcreteTypeExpression)]
pub trait PtConcreteTypeExpressionLike {
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
