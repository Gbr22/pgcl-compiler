use std::collections::VecDeque;

use crate::{parser::{tree::{TreeNode, TreeNodeLike, ParseError, get_start_index, get_end_index, get_range}, grammar::GrammarLike, grammars::{uniform_declaration::UniformDeclarationGrammar, function_declaration::{FunctionDeclarationGrammar}}, nodes::expressions::expr::Expression, tree_nodes::TreeNodes}, lexer::types::{keywords::RETURN, token_type::TokenType}, common::range::Range};

use super::statement::{StatementLike, Statement};

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    range: Range,
    expr: Box<TreeNode>
}

impl ReturnStatement {
    pub fn parse(nodes: TreeNodes) -> TreeNode {
        let nodes = nodes.vec;
        let range = get_range(&nodes).unwrap_or(Range::null());

        let mut queue: VecDeque<TreeNode> = nodes.into();
        let keyword_error = ParseError::at(range, format!("Expected return keyword."));
        let return_keyword = queue.pop_front();
        let Some(return_keyword) = return_keyword else {
            return keyword_error.into();
        };
        if !return_keyword.is_keyword(RETURN) {
            return return_keyword.into();
        }
        let semi_error = ParseError::at(range, format!("Semicolon expected at end of return statement."));
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