use crate::{parser::{grammar::GrammarLike, tree::TreeNode, nodes::{document::Document, statements::simple_statement::SimpleStatement}}, lexer::types::token_type::TokenType};

pub struct SimpleStatementGrammar {}

impl GrammarLike for SimpleStatementGrammar {
    fn next_match_start(&self, nodes: &[TreeNode]) -> Option<usize> {
        if nodes.len() == 0 {
            return None
        }
        for (index, node) in nodes.iter().enumerate() {
            if let TreeNode::Statement(simple_statement) = node {
                continue;
            }
            return Some(index);
        }

        None
    }
    fn next_match_end(&self, nodes: &[TreeNode], start_index: usize) -> Option<usize> {
        for (index, node) in nodes.iter().enumerate() {
            if index < start_index {
                continue;
            }
            if node.is_token_type(TokenType::Semicolon) {
                return Some(index);
            }
        }

        None
    }
    fn construct(&self, nodes: Vec<TreeNode>) -> TreeNode {
        let statement = SimpleStatement::parse(nodes);

        statement
    }
}