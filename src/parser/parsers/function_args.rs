use crate::parser::grammars::function_arg::FunctionArgGrammar;
use crate::parser::nodes::function_args::FunctionArgs;
use crate::parser::nodes::function_call_args::FunctionCallArgs;
use crate::parser::{parse::Parser, tree::TreeNode, tree_nodes::TreeNodes};
use crate::process_grammars;
pub struct FunctionArgsParser {}

impl Parser for FunctionArgsParser {
    fn parse(nodes: TreeNodes) -> TreeNode {
        let range = nodes.range;

        let nodes = process_grammars! { nodes [
            FunctionArgGrammar
        ] };

        // TODO: assert that the only children are FunctionArg structs

        let fn_call_args = FunctionArgs {
            args: nodes.into_vec(),
            range,
        };
        TreeNode::FunctionArgs(fn_call_args)
    }
}
