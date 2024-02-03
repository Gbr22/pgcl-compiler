use crate::{
    lexer::types::{keywords::is_keyword, token_type::TokenType},
    parser::{
        nodes::{function_arg::FunctionArg, function_call_arg::FunctionCallArg, types::{typ, type_arg::TypeArg}}, parse::Parser, parsers::types::typ::TypeParser, tree::TreeNode, tree_nodes::TreeNodes
    }, pop_front_node, try_nodes_into_one,
};

pub struct TypeArgParser {}

impl Parser for TypeArgParser {
    fn parse(mut nodes: TreeNodes) -> TreeNode {
        let range = nodes.range;

        let has_comma = match nodes.last() {
            None => false,
            Some(node) => node.is_token_type(TokenType::Comma),
        };

        if has_comma {
            nodes.pop_back();
        }

        let node = TypeParser::parse(nodes);

        let arg = TypeArg {
            value: Box::new(node),
            range
        };

        TreeNode::TypeArg(arg)
    }
}
