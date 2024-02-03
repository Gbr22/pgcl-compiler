use crate::{
    lexer::types::token_type::TokenType,
    parser::{
        grammar::GrammarLike,
        nodes::{document::Document, statements::simple_statement::SimpleStatement},
        tree::TreeNode,
        tree_nodes::TreeNodes,
    },
};

pub struct SimpleStatementGrammar {}

impl GrammarLike for SimpleStatementGrammar {
    fn next_match_start(&self, nodes: &TreeNodes) -> Option<usize> {
        if nodes.len() == 0 {
            return None;
        }
        for (index, node) in nodes.iter().enumerate() {
            if let TreeNode::Statement(simple_statement) = node {
                continue;
            }
            return Some(index);
        }

        None
    }
    fn next_match_end(&self, nodes: &TreeNodes, start_index: usize) -> Option<usize> {
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
    fn construct(&self, nodes: TreeNodes) -> TreeNode {
        let statement = SimpleStatement::parse(nodes);

        statement
    }
}
