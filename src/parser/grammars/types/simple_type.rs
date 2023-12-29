use crate::{parser::{grammar::GrammarLike, tree::{TreeNode, ParseError}, nodes::{document::Document, types::simple_type::SimpleType}}, lexer::types::{token_type::TokenType, keywords::is_keyword}};

pub struct SimpleTypeGrammar {}

impl GrammarLike for SimpleTypeGrammar {
    fn next_match_at(&self, nodes: &[TreeNode]) -> Option<usize> {
        for (index, node) in nodes.iter().enumerate() {
            let TreeNode::Token(token) = node else {
                continue;
            };
            if token.typ != TokenType::Identifier || is_keyword(&token.string) {
                continue;
            }

            return Some(index)
        }

        None
    }
    fn find_match_end(&self, nodes: &[TreeNode], start_index: usize) -> Option<usize> {
        Some(start_index)
    }
    fn construct(&self, nodes: Vec<TreeNode>) -> TreeNode {
        if nodes.len() > 1 {
            return ParseError::from_nodes(&nodes, format!("Multiple types detected where only one is expected.")).into();
        }
        if nodes.len() == 0 {
            return ParseError::from_nodes(&nodes, format!("Type expected.")).into();
        }
        SimpleType::parse(nodes[0].clone())
    }
}