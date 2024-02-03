use crate::parser::parsers::uniform_declaration::UniformDeclarationParser;
use crate::parser::tree_nodes::TreeNodes;
use crate::use_parser;
use crate::{
    lexer::types::{keywords::UNIFORM, token_type::TokenType},
    parser::{grammar::GrammarLike, parse::Parser, tree::TreeNode},
};

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
    use_parser!(UniformDeclarationParser);

    fn next_match_start(&self, nodes: &TreeNodes) -> Option<usize> {
        for (index, node) in nodes.iter().enumerate() {
            let TreeNode::Token(token) = node else {
                continue;
            };
            if &token.string != UNIFORM {
                continue;
            }
            return Some(index);
        }

        None
    }

    fn next_match_end(&self, nodes: &TreeNodes, start_index: usize) -> Option<usize> {
        for (index, item) in nodes.iter().enumerate() {
            if index <= start_index {
                continue;
            }

            if can_end_uniform_search(item) {
                return Some(index);
            }
        }

        None
    }
}
