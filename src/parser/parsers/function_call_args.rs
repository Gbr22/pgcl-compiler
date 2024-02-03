use crate::parser::grammars::function_call_arg::FunctionCallArgGrammar;
use crate::parser::nodes::function_call_args::FunctionCallArgs;
use crate::parser::{parse::Parser, tree::TreeNode, tree_nodes::TreeNodes};
use crate::process_grammars;
pub struct FunctionCallArgsParser {}

impl Parser for FunctionCallArgsParser {
    fn parse(nodes: TreeNodes) -> TreeNode {
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
