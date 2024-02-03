use crate::{
    lexer::types::token_type::TokenType,
    parser::{
        nodes::types::type_arg::TypeArg, parse::Parser, parsers::types::typ::TypeParser,
        tree::TreeNode, tree_nodes::TreeNodes,
    },
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
            range,
        };

        TreeNode::TypeArg(arg)
    }
}
