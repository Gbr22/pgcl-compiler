use super::function_call::FunctionCall;
use super::value_access::{PtValueAccess, ValueAccess};

use crate::parser::program_tree::program_tree::{
    CurrentContext, PtError, RootContextMutRef, TryIntoPt,
};
use crate::parser::tree::TreeNodeLike;

trait_enum! {
    #[derive(Debug, Clone)]
    pub enum Expression: ExpressionLike {
        ValueAccess,
        FunctionCall
    }
}

pub trait ExpressionLike {
    fn to_node_like(&self) -> Box<&dyn TreeNodeLike>;
}

impl TreeNodeLike for Expression {
    fn get_range(&self) -> crate::common::range::Range {
        self.to_node_like().get_range()
    }
    fn children(&self) -> Vec<&crate::parser::tree::TreeNode> {
        self.to_node_like().children()
    }
}

impl Expression {}

#[derive(Debug, Clone)]
pub enum PtExpression {
    ValueAccess(PtValueAccess),
}

impl TryIntoPt<PtExpression> for Expression {
    fn try_into_pt(
        self,
        root_context: RootContextMutRef,
        context: &CurrentContext,
    ) -> Result<PtExpression, PtError> {
        match self {
            Expression::ValueAccess(v) => Ok(PtExpression::ValueAccess(
                v.try_into_pt(root_context, context)?,
            )),
            Expression::FunctionCall(f) => Err(PtError::in_at(
                &context.uri,
                f.range,
                "TODO: Function call".to_string(),
            )),
        }
    }
}
