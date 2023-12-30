use crate::{parser::{grammar::GrammarLike, tree::TreeNode, nodes::{document::Document, statements::{simple_statement::SimpleStatement, ret::ReturnStatement}}}, lexer::types::{token_type::TokenType, keywords::RETURN}};

pub struct ReturnStatementGrammar {}

impl GrammarLike for ReturnStatementGrammar {
    fn next_match_start(&self, nodes: &[TreeNode]) -> Option<usize> {
        for (index, node) in nodes.iter().enumerate() {
            if node.is_keyword(RETURN) {
                return Some(index);
            }
        }

        None
    }
    fn next_match_end(&self, nodes: &[TreeNode], _start_index: usize) -> Option<usize> {
        Some(nodes.len()-1)
    }
    fn construct(&self, nodes: Vec<TreeNode>) -> TreeNode {
        let statement = ReturnStatement::parse(nodes);

        statement
    }
}