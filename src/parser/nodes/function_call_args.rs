

use crate::{parser::{tree::{TreeNode, ParseError, TreeNodeLike, get_start_index, get_end_index}, grammars::function_call_arg::FunctionCallArgGrammar}, process_grammars};



#[derive(Debug, Clone)]
pub struct FunctionCallArgs {
    start_index: usize,
    end_index: usize,
    args: Vec<TreeNode>
}

impl FunctionCallArgs {
    pub fn parse(nodes: Vec<TreeNode>) -> TreeNode {
        let start_index = get_start_index(&nodes).unwrap_or_default();
        let end_index = get_end_index(&nodes).unwrap_or_default();

        let nodes = process_grammars! { nodes [
            FunctionCallArgGrammar
        ] };

        // TODO: assert that the only children are FunctionCallArg structs

        let fn_call_args = FunctionCallArgs {
            start_index,
            end_index,
            args: nodes
        };
        TreeNode::FunctionCallArgs(fn_call_args)
    }
}

impl TreeNodeLike for FunctionCallArgs {
    fn get_start_index(&self) -> usize {
        self.start_index
    }
    fn get_end_index(&self) -> usize {
        self.end_index
    }
    fn get_errors(&self) -> Vec<ParseError> {
        let errors: Vec<ParseError> = self.args.iter()
            .flat_map(|arg|arg.get_errors())
            .collect();

        errors
    }
}
