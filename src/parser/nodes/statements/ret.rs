use std::collections::VecDeque;

use crate::{parser::{tree::{TreeNode, TreeNodeLike, ParseError, get_start_index, get_end_index}, grammar::GrammarLike, grammars::{uniform_declaration::UniformDeclarationGrammar, function_declaration::{FunctionDeclarationGrammar}}, nodes::expressions::expr::Expression}, lexer::types::{keywords::RETURN, token_type::TokenType}};

use super::statement::{StatementLike, Statement};

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    start_index: usize,
    end_index: usize,
    expr: Box<TreeNode>
}

impl ReturnStatement {
    pub fn parse(nodes: Vec<TreeNode>) -> TreeNode {
        let start_index = get_start_index(&nodes)
            .unwrap_or_default();
        let end_index = get_end_index(&nodes)
            .unwrap_or_default();

        let mut queue: VecDeque<TreeNode> = nodes.into();
        let keyword_error = ParseError::at(start_index, end_index, format!("Expected return keyword."));
        let return_keyword = queue.pop_front();
        let Some(return_keyword) = return_keyword else {
            return keyword_error.into();
        };
        if !return_keyword.is_keyword(RETURN) {
            return return_keyword.into();
        }
        let semi_error = ParseError::at(start_index, end_index, format!("Semicolon expected at end of return statement."));
        let semi_colon = queue.pop_back();
        let Some(semi_colon) = semi_colon else {
            return semi_error.into();
        };
        if !semi_colon.is_token_type(TokenType::Semicolon) {
            return semi_error.into();
        }

        let nodes: Vec<TreeNode> = queue.into();
        let expr = Expression::parse(nodes);

        let statement = ReturnStatement {
            start_index,
            end_index,
            expr: Box::new(expr)
        };

        TreeNode::Statement(Statement::ReturnStatement(statement))
    }
}

impl TreeNodeLike for ReturnStatement {
    fn get_start_index(&self) -> usize {
        self.start_index
    }
    fn get_end_index(&self) -> usize {
        self.end_index
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