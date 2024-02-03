use crate::{
    lexer::types::{keywords::is_keyword, token_type::TokenType},
    parser::{
        nodes::types::{simple::SimpleType, typ::Type},
        parse::Parser,
        tree::{ParseError, TreeNode},
        tree_nodes::TreeNodes,
    },
    try_nodes_into_one, try_nodes_into_one_with_message,
};

pub struct SimpleTypeParser {}

impl Parser for SimpleTypeParser {
    fn parse(nodes: TreeNodes) -> TreeNode {
        try_nodes_into_one_with_message!(
            let node from nodes;
            =0: "Multiple types found where only one is expected.";
            >1: "Expected type.";
        );
        let range = node.get_range();

        let id_error = ParseError::at(range, format!("Expected simple type, got {:?}.", node));

        let TreeNode::Token(token) = node else {
            return id_error.into();
        };
        if token.typ != TokenType::Identifier || is_keyword(&token.string) {
            return id_error.into();
        }

        let name = token.string;

        let typ = SimpleType { name, range };

        TreeNode::Type(Type::SimpleType(typ))
    }
}
