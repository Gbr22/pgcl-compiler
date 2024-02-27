use crate::{
    common::range::Range,
    parser::{
        program_tree::program_tree::{CurrentContext, PtError, RootContextMutRef, TryIntoPt},
        tree::{TreeNode, TreeNodeLike},
    },
};

use super::expr::{ExpressionLike, PtExpression};

#[derive(Debug, Clone)]
pub struct AstFunctionCall {
    pub name: String,
    pub name_range: Range,
    pub range: Range,
    pub args: Box<TreeNode>,
}

impl TreeNodeLike for AstFunctionCall {
    fn get_range(&self) -> Range {
        self.range
    }

    fn children(&self) -> Vec<&TreeNode> {
        vec![&self.args]
    }
}

impl ExpressionLike for AstFunctionCall {
    fn to_node_like(&self) -> Box<&dyn TreeNodeLike> {
        Box::new(self)
    }
}

#[derive(Debug, Clone)]
pub struct PtFunctionCall {
    pub name: String,
    pub name_range: Range,
    pub range: Range,
    pub args: Vec<PtExpression>,
}

impl TryIntoPt<PtFunctionCall> for AstFunctionCall {
    fn try_into_pt(
        self,
        root_context: RootContextMutRef,
        context: &CurrentContext,
    ) -> Result<PtFunctionCall, PtError> {
        let name = self.name;
        let name_range = self.name_range;
        let range = self.range;

        let args = match *self.args {
            TreeNode::FunctionCallArgs(args) => args.try_into_pt(root_context, context)?,
            node => {
                return Err(PtError::in_at(
                    &context.uri,
                    node.get_range(),
                    format!("Expected function args."),
                ));
            }
        };

        Ok(PtFunctionCall {
            name,
            name_range,
            range,
            args,
        })
    }
}
