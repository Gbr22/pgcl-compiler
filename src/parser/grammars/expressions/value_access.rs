use crate::{parser::{grammar::GrammarLike, tree::{TreeNode, ParseError}, nodes::{document::Document, statements::{simple_statement::SimpleStatement, ret::ReturnStatement}, expressions::{function_call::FunctionCall, value_access::ValueAccess}}, match_brackets::find_bracket_end, brackets::round_bracket}, lexer::types::{token_type::TokenType, keywords::{RETURN, is_keyword}}};

#[derive(Clone)]
pub struct ValueAccessGrammar {}

impl GrammarLike for ValueAccessGrammar {
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
    fn find_match_end(&self, _nodes: &[TreeNode], start_index: usize) -> Option<usize> {
        Some(start_index)   
    }
    fn construct(&self, nodes: Vec<TreeNode>) -> TreeNode {
        if nodes.len() == 0 {
            return ParseError::from_nodes(&nodes, format!("Identifier expected.")).into();
        }
        if nodes.len() > 1 {
            return ParseError::from_nodes(&nodes, format!("Too many items in value access. Expected one.")).into();
        }

        let node = nodes.into_iter().next().expect("Nodes empty in `ValueAccessGrammar::construct`.");
        let node = ValueAccess::parse(node);

        node
    }
}