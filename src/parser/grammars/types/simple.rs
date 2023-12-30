use crate::{parser::{grammar::GrammarLike, tree::{TreeNode, ParseError}, nodes::{document::Document, types::simple::SimpleType}, tree_nodes::TreeNodes}, lexer::types::{token_type::TokenType, keywords::is_keyword}};

pub struct SimpleTypeGrammar {}

impl GrammarLike for SimpleTypeGrammar {
    fn next_match_start(&self, nodes: &TreeNodes) -> Option<usize> {
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
    fn next_match_end(&self, _nodes: &TreeNodes, start_index: usize) -> Option<usize> {
        Some(start_index)
    }
    fn construct(&self, nodes: TreeNodes) -> TreeNode {
        if nodes.len() > 1 {
            return ParseError::at(nodes.range, format!("Multiple types detected where only one is expected.")).into();
        }
        if nodes.len() == 0 {
            return ParseError::at(nodes.range, format!("Type expected.")).into();
        }
        SimpleType::parse(nodes.into_first().unwrap())
    }
}