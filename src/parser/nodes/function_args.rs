use crate::{
    common::range::Range,
    parser::{
        program_tree::program_tree::{CurrentContext, PtError, RootContextMutRef, TryIntoPt},
        tree::{TreeNode, TreeNodeLike},
    },
};

use super::function_arg::PtFunctionArg;

#[derive(Debug, Clone)]
pub struct FunctionArgs {
    pub args: Vec<TreeNode>,
    pub range: Range,
}

impl TreeNodeLike for FunctionArgs {
    fn get_range(&self) -> Range {
        self.range
    }
    fn children(&self) -> Vec<&TreeNode> {
        self.args.iter().collect()
    }
}

impl TryIntoPt<Vec<PtFunctionArg>> for FunctionArgs {
    fn try_into_pt(
        self,
        root_context: RootContextMutRef,
        context: &CurrentContext,
    ) -> Result<Vec<PtFunctionArg>, PtError> {
        let mut vec: Vec<PtFunctionArg> = vec![];
        for arg in self.args.into_iter() {
            let TreeNode::FunctionArg(arg) = arg else {
                return Err(PtError::in_at(
                    &context.uri,
                    arg.get_range(),
                    "Expected argument.",
                ));
            };
            let arg = arg.try_into_pt(root_context.clone(), context)?;
            vec.push(arg);
        }

        Ok(vec)
    }
}
