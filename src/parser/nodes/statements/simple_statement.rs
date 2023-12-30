use std::collections::VecDeque;

use crate::{parser::{tree::{TreeNode, TreeNodeLike, ParseError, get_start_index, get_end_index, get_range}, grammar::GrammarLike, grammars::{uniform_declaration::UniformDeclarationGrammar, function_declaration::{FunctionDeclarationGrammar}}, nodes::expressions::expr::Expression, tree_nodes::TreeNodes}, lexer::types::token_type::TokenType, common::range::Range};

use super::statement::{Statement, StatementLike};

// Semicolon delimited statement
#[derive(Debug, Clone)]
pub struct SimpleStatement {
    range: Range,
    expr: Box<TreeNode>
}

impl SimpleStatement {
    pub fn parse(nodes: TreeNodes) -> TreeNode {
        let range = nodes.range;

        let mut queue: VecDeque<TreeNode> = nodes.vec.into();
        let semi_error = ParseError::at(range, format!("Semicolon expected at end of statement."));
        let semi_colon = queue.pop_back();
        let Some(semi_colon) = semi_colon else {
            return semi_error.into();
        };
        if !semi_colon.is_token_type(TokenType::Semicolon) {
            return semi_error.into();
        }
        let nodes: Vec<TreeNode> = queue.into();
        let nodes = TreeNodes::new(range, nodes);

        let expr = Expression::parse(nodes);

        let statement = SimpleStatement {
            range,
            expr: Box::new(expr)
        };

        TreeNode::Statement(Statement::SimpleStatement(statement))
    }
}

impl TreeNodeLike for SimpleStatement {
    fn get_range(&self) -> Range {
        self.range
    }
    fn get_errors(&self) -> Vec<ParseError> {
        self.expr.get_errors()
    }
}

impl StatementLike for SimpleStatement {
    fn to_node_like(&self) -> Box<&dyn TreeNodeLike> {
        Box::new(self)
    }
}