

use crate::{parser::{tree::{TreeNode, ParseError, TreeNodeLike, get_start_index, get_end_index, get_range}, grammars::function_call_arg::FunctionCallArgGrammar}, process_grammars, common::range::Range};



#[derive(Debug, Clone)]
pub struct FunctionCallArgs {
    args: Vec<TreeNode>,
    range: Range
}

impl FunctionCallArgs {
    pub fn parse(nodes: Vec<TreeNode>) -> TreeNode {
        let range = get_range(&nodes).unwrap_or(Range::null());

        let nodes = process_grammars! { nodes [
            FunctionCallArgGrammar
        ] };

        // TODO: assert that the only children are FunctionCallArg structs

        let fn_call_args = FunctionCallArgs {
            args: nodes,
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
        let errors: Vec<ParseError> = self.args.iter()
            .flat_map(|arg|arg.get_errors())
            .collect();

        errors
    }
}
