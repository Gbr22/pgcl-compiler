use enum_dispatch::enum_dispatch;

use super::function_call::{AstFunctionCall, PtFunctionCall};
use super::value_access::{AstValueAccess, PtValueAccess};

use crate::parser::program_tree::program_tree::{
    CurrentContext, PtError, RootContextMutRef, TryIntoPt,
};
use crate::parser::tree::TreeNodeLike;

#[derive(Debug, Clone)]
#[enum_dispatch]
pub enum Expression {
    ValueAccess(AstValueAccess),
    FunctionCall(AstFunctionCall),
}

#[enum_dispatch(Expression)]
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
    FunctionCall(PtFunctionCall),
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
            Expression::FunctionCall(f) => Ok(PtExpression::FunctionCall(
                f.try_into_pt(root_context, context)?,
            )),
        }
    }
}
