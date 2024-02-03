use crate::parser::grammars::types::type_arg::TypeArgGrammar;

use crate::parser::nodes::types::type_args::TypeArgs;
use crate::parser::{parse::Parser, tree::TreeNode, tree_nodes::TreeNodes};
use crate::process_grammars;
pub struct TypeArgsParser {}

impl Parser for TypeArgsParser {
    fn parse(nodes: TreeNodes) -> TreeNode {
        let range = nodes.range;

        let nodes = process_grammars! { nodes [
            TypeArgGrammar
        ] };

        // TODO: assert that the only children are FunctionArg structs

        let fn_call_args = TypeArgs {
            args: nodes.into_vec(),
            range,
        };
        TreeNode::TypeArgs(fn_call_args)
    }
}
