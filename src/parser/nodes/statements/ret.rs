use crate::{
    common::range::Range,
    parser::{
        nodes::expressions::expr::PtExpression,
        program_tree::program_tree::{CurrentContext, PtError, RootContextMutRef, TryIntoPt},
        tree::{TreeNode, TreeNodeLike},
    },
};

use super::statement::StatementLike;

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub range: Range,
    pub expr: Box<TreeNode>,
}

impl ReturnStatement {}

impl TreeNodeLike for ReturnStatement {
    fn get_range(&self) -> Range {
        self.range
    }
    fn children(&self) -> Vec<&TreeNode> {
        vec![&self.expr]
    }
}

impl StatementLike for ReturnStatement {
    fn to_node_like(&self) -> Box<&dyn TreeNodeLike> {
        Box::new(self)
    }
}

#[derive(Debug, Clone)]
pub struct PtReturnStatement {
    pub range: Range,
    pub expr: PtExpression,
}

impl TryIntoPt<PtReturnStatement> for ReturnStatement {
    fn try_into_pt(
        self,
        root_context: RootContextMutRef,
        context: &CurrentContext,
    ) -> Result<PtReturnStatement, PtError> {
        let range = self.range;

        match *self.expr {
            TreeNode::Expression(e) => {
                let expr = e.try_into_pt(root_context, context)?;
                Ok(PtReturnStatement { range, expr })
            }
            _ => Err(PtError::in_at(
                &context.uri,
                range,
                "Expected expression.".to_string(),
            )),
        }
    }
}
