use crate::{
    lexer::types::token_type::TokenType,
    parser::{
        nodes::expressions::{expr::Expression, value_access::AstValueAccess},
        parse::Parser,
        tree::{ParseError, TreeNode},
        tree_nodes::TreeNodes,
    },
    try_nodes_into_one,
};

pub struct ValueAccessParser {}

impl Parser for ValueAccessParser {
    fn parse(nodes: TreeNodes) -> TreeNode {
        try_nodes_into_one!(let node from nodes);

        let range = node.get_range();

        let id_error = ParseError::at(range, "Expected identifier.".to_string());
        let TreeNode::Token(name) = node else {
            return id_error.into();
        };
        if name.typ != TokenType::Identifier {
            return id_error.into();
        }
        let name = name.string.to_owned();

        let value_access = AstValueAccess { name, range };
        TreeNode::Expression(Expression::ValueAccess(value_access))
    }
}
