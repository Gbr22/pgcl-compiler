use super::statement::StatementLike;
use crate::{
    common::range::Range,
    parser::{nodes::expressions::expr::PtExpression, program_tree::program_tree::{CurrentContext, PtError, RootContextMutRef, TryIntoPt}, tree::{TreeNode, TreeNodeLike}},
};

// Semicolon delimited statement
#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub range: Range,
    pub expr: Box<TreeNode>,
}

impl TreeNodeLike for ExpressionStatement {
    fn get_range(&self) -> Range {
        self.range
    }
    fn children(&self) -> Vec<&TreeNode> {
        vec![&self.expr]
    }
}

impl StatementLike for ExpressionStatement {
    fn to_node_like(&self) -> Box<&dyn TreeNodeLike> {
        Box::new(self)
    }
}

#[derive(Debug, Clone)]
pub struct PtExpressionStatement {
    pub range: Range,
    pub expr: PtExpression
}

impl TryIntoPt<PtExpressionStatement> for ExpressionStatement {
    fn try_into_pt(
        self,
        root_context: RootContextMutRef,
        context: &CurrentContext,
    ) -> Result<PtExpressionStatement, PtError> {
        let range = self.range;

        match *self.expr {
            TreeNode::Expression(e)=>{
                let expr = e.try_into_pt(root_context, context)?;
                Ok(PtExpressionStatement {
                    range,
                    expr,
                })
            }
            _ => {
                return Err(PtError::in_at(&context.uri, range, format!("Expected expression.")))
            }
        }
    }
}