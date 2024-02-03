use crate::{
    lexer::types::{keywords::RETURN, token_type::TokenType},
    parser::{
        nodes::statements::{ret::ReturnStatement, statement::Statement},
        parse::Parser,
        parsers::expressions::expr::ExpressionParser,
        tree::TreeNode,
        tree_nodes::TreeNodes,
    },
    pop_back_node, pop_front_node,
};

pub struct ReturnStatementParser {}

impl Parser for ReturnStatementParser {
    fn parse(mut nodes: TreeNodes) -> TreeNode {
        let range = nodes.range;

        pop_front_node!(
            nodes,
            "Expected `return` keyword.",
            Some(TreeNode::Token(return_keyword)),
            return_keyword.typ == TokenType::Identifier && return_keyword.string == RETURN
        );

        pop_back_node!(
            nodes,
            "Semicolon expected at end of return statement.",
            Some(TreeNode::Token(semi_colon)),
            semi_colon.typ == TokenType::Semicolon
        );

        let expr = ExpressionParser::parse(nodes);

        let statement = ReturnStatement {
            range,
            expr: Box::new(expr),
        };

        TreeNode::Statement(Statement::ReturnStatement(statement))
    }
}
