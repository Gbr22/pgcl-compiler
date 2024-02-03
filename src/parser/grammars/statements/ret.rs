use crate::{
    lexer::types::{keywords::RETURN, token_type::TokenType},
    parser::{
        grammar::GrammarLike,
        nodes::{
            document::Document,
            statements::{ret::ReturnStatement, simple_statement::SimpleStatement},
        },
        tree::TreeNode,
        tree_nodes::TreeNodes,
    },
};

pub struct ReturnStatementGrammar {}

impl GrammarLike for ReturnStatementGrammar {
    fn next_match_start(&self, nodes: &TreeNodes) -> Option<usize> {
        for (index, node) in nodes.iter().enumerate() {
            if node.is_keyword(RETURN) {
                return Some(index);
            }
        }

        None
    }
    fn next_match_end(&self, nodes: &TreeNodes, _start_index: usize) -> Option<usize> {
        Some(nodes.len() - 1)
    }
    fn construct(&self, nodes: TreeNodes) -> TreeNode {
        let statement = ReturnStatement::parse(nodes);

        statement
    }
}
