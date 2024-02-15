use crate::{
    common::range::Range,
    parser::{program_tree::program_tree::{CurrentContext, PtError, RootContextRef, TryIntoPt}, tree::{TreeNode, TreeNodeLike}},
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
        root_context: RootContextRef,
        context: &CurrentContext,
    ) -> Result<Vec<PtFunctionArg>, PtError> {
        let mut vec: Vec<PtFunctionArg> = vec![];
        for arg in self.args.into_iter() {
            let TreeNode::FunctionArg(arg) = arg else {
                return Err(PtError {
                    range: Some(arg.get_range()),
                    message: format!("Expected argument."),
                })
            };
            let arg = arg.try_into_pt(root_context.clone(), context)?;
            vec.push(arg);
        };

        Ok(vec)
    }
}