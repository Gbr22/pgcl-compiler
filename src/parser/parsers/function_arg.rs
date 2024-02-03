use crate::{
    lexer::types::{keywords::is_keyword, token_type::TokenType},
    parser::{
        nodes::function_arg::FunctionArg, parse::Parser, parsers::types::typ::TypeParser,
        tree::TreeNode, tree_nodes::TreeNodes,
    },
    pop_front_node,
};

pub struct FunctionArgParser {}

impl Parser for FunctionArgParser {
    fn parse(mut nodes: TreeNodes) -> TreeNode {
        let range = nodes.range;

        let has_comma = match nodes.last() {
            None => false,
            Some(node) => node.is_token_type(TokenType::Comma),
        };

        if has_comma {
            nodes.pop_back();
        }

        pop_front_node!(
            nodes,
            "Missing argument name",
            Some(TreeNode::Token(name)),
            name.typ == TokenType::Identifier && !is_keyword(&name.string)
        );

        let name = name.string;

        pop_front_node!(
            nodes,
            "Expected colon",
            Some(TreeNode::Token(colon)),
            colon.typ == TokenType::Colon
        );

        let type_nodes = nodes;

        let typ = TypeParser::parse(type_nodes);

        let arg = FunctionArg {
            name,
            typ: Box::new(typ),
            range,
        };

        TreeNode::FunctionArg(arg)
    }
}
