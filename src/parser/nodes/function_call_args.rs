use crate::{
    common::range::Range,
    parser::{
        grammars::function_call_arg::FunctionCallArgGrammar,
        tree::{get_end_index, get_range, get_start_index, ParseError, TreeNode, TreeNodeLike},
        tree_nodes::TreeNodes,
    },
    process_grammars,
};

#[derive(Debug, Clone)]
pub struct FunctionCallArgs {
    args: Vec<TreeNode>,
    range: Range,
}

impl FunctionCallArgs {
    pub fn parse(nodes: TreeNodes) -> TreeNode {
        let range = nodes.range;

        let nodes = process_grammars! { nodes [
            FunctionCallArgGrammar
        ] };

        // TODO: assert that the only children are FunctionCallArg structs

        let fn_call_args = FunctionCallArgs {
            args: nodes.into_vec(),
            range,
        };
        TreeNode::FunctionCallArgs(fn_call_args)
    }
}

impl TreeNodeLike for FunctionCallArgs {
    fn get_range(&self) -> Range {
        self.range
    }
    fn get_errors(&self) -> Vec<ParseError> {
        let errors: Vec<ParseError> = self.args.iter().flat_map(|arg| arg.get_errors()).collect();

        errors
    }
}
