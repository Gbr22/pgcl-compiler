use crate::{
    lexer::types::token_type::TokenType,
    parser::{
        nodes::statements::{simple::ExpressionStatement, statement::Statement},
        parse::Parser,
        parsers::expressions::expr::ExpressionParser,
        tree::TreeNode,
        tree_nodes::TreeNodes,
    },
    pop_back_node,
};

pub struct SimpleStatementParser {}

impl Parser for SimpleStatementParser {
    fn parse(mut nodes: TreeNodes) -> TreeNode {
        let range = nodes.range;

        pop_back_node!(
            nodes,
            "Semicolon expected at end of statement.",
            Some(TreeNode::Token(semi_colon)),
            semi_colon.typ == TokenType::Semicolon
        );

        let expr = ExpressionParser::parse(nodes);

        let statement = ExpressionStatement {
            range,
            expr: Box::new(expr),
        };

        TreeNode::Statement(Statement::ExpressionStatement(statement))
    }
}
