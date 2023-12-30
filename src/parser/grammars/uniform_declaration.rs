use crate::parser::nodes::uniform_declaration::UniformDeclaration;
use crate::{parser::{tree::TreeNode, grammar::GrammarLike}, lexer::types::{token_type::TokenType, keywords::UNIFORM}};

pub struct UniformDeclarationGrammar {}

fn can_end_uniform_search(node: &TreeNode) -> bool {
    if node.is_token_type(TokenType::Semicolon) {
        return true;
    };
    if node.is_keyword("fn") {
        return true;
    }
    if node.is_keyword("uniform") {
        return true;
    }
    if let TreeNode::Token(token) = node {
        if token.typ == TokenType::LineComment {
            return true;
        }
    }

    false
}

impl GrammarLike for UniformDeclarationGrammar {
    fn next_match_start(&self, nodes: &[TreeNode]) -> Option<usize> {
        for (index, node) in nodes.iter().enumerate() {
            let TreeNode::Token(token) = node else {
                continue
            };
            if &token.string != UNIFORM {
                continue;
            }
            return Some(index);
        }

        None
    }

    fn next_match_end(&self, nodes: &[TreeNode], start_index: usize) -> Option<usize> {
        for (index, item) in nodes.iter().enumerate() {
            if index <= start_index {
                continue;
            }

            if can_end_uniform_search(&item) {
                return Some(index)
            }
        }

        None
    }

    fn construct(&self, nodes: Vec<TreeNode>) -> TreeNode {
        UniformDeclaration::parse(nodes)
    }
}