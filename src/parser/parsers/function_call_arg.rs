use crate::{
    lexer::types::token_type::TokenType,
    parser::{
        nodes::function_call_arg::FunctionCallArg, parse::Parser, tree::TreeNode,
        tree_nodes::TreeNodes,
    },
};

use super::expressions::expr::ExpressionParser;

pub struct FunctionCallArgParser {}

impl Parser for FunctionCallArgParser {
    fn parse(mut nodes: TreeNodes) -> TreeNode {
        let range = nodes.range;

        let has_comma = match nodes.last() {
            None => false,
            Some(node) => node.is_token_type(TokenType::Comma),
        };

        if has_comma {
            nodes.pop_back();
        }

        let expr = ExpressionParser::parse(nodes);

        let arg = FunctionCallArg {
            expr: Box::new(expr),
            range,
        };

        TreeNode::FunctionCallArg(arg)
    }
}
