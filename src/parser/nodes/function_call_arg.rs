use crate::common::range::Range;
use crate::parser::program_tree::program_tree::{
    CurrentContext, PtError, RootContextMutRef, TryIntoPt,
};
use crate::parser::tree::{TreeNode, TreeNodeLike};

use super::expressions::expr::{Expression, PtExpression};

#[derive(Debug, Clone)]
pub struct FunctionCallArg {
    pub expr: Box<TreeNode>,
    pub range: Range,
}

impl TreeNodeLike for FunctionCallArg {
    fn get_range(&self) -> Range {
        self.range
    }
    fn children(&self) -> Vec<&TreeNode> {
        vec![&self.expr]
    }
}

impl TryIntoPt<PtExpression> for FunctionCallArg {
    fn try_into_pt(
        self,
        root_context: RootContextMutRef,
        context: &CurrentContext,
    ) -> Result<PtExpression, PtError> {
        let expr = match *self.expr {
            TreeNode::Expression(expr) => expr.try_into_pt(root_context, context)?,
            node => {
                return Err(PtError::in_at(
                    &context.uri,
                    node.get_range(),
                    format!("Expected expression."),
                ));
            }
        };

        Ok(expr)
    }
}
