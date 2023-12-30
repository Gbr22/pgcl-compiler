use std::collections::VecDeque;

use crate::{parser::{tree::{TreeNode, TreeNodeLike, ParseError, get_start_index, get_end_index, get_range}, grammar::GrammarLike, grammars::{uniform_declaration::UniformDeclarationGrammar, function_declaration::{FunctionDeclarationGrammar}}, nodes::expressions::expr::Expression, tree_nodes::TreeNodes}, lexer::types::{keywords::RETURN, token_type::TokenType}, common::range::Range};

use super::statement::{StatementLike, Statement};
use crate::pop_back_node;
use crate::pop_front_node;

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    range: Range,
    expr: Box<TreeNode>
}

impl ReturnStatement {
    pub fn parse(mut nodes: TreeNodes) -> TreeNode {
        let range = nodes.range;

        pop_front_node!(
            nodes,
            "Expected `return` keyword.",
            Some(TreeNode::Token(return_keyword)),
            return_keyword.typ == TokenType::Identifier
            && return_keyword.string == RETURN
        );
        
        pop_back_node!(
            nodes,
            "Semicolon expected at end of return statement.",
            Some(TreeNode::Token(semi_colon)),
            semi_colon.typ == TokenType::Semicolon
        );

        let expr = Expression::parse(nodes);

        let statement = ReturnStatement {
            range,
            expr: Box::new(expr)
        };

        TreeNode::Statement(Statement::ReturnStatement(statement))
    }
}

impl TreeNodeLike for ReturnStatement {
    fn get_range(&self) -> Range {
        self.range
    }
    fn get_errors(&self) -> Vec<ParseError> {
        self.expr.get_errors()
    }
}

impl StatementLike for ReturnStatement {
    fn to_node_like(&self) -> Box<&dyn TreeNodeLike> {
        Box::new(self)
    }
}