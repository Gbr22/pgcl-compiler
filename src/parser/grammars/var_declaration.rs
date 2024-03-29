use crate::lexer::types::keywords::FN;
use crate::parser::parsers::var_declaration::VarDeclarationParser;
use crate::parser::tree_nodes::TreeNodes;
use crate::use_parser;
use crate::{
    lexer::types::{keywords::VAR, token_type::TokenType},
    parser::{grammar::GrammarLike, parse::Parser, tree::TreeNode},
};

pub struct VarDeclarationGrammar {}

fn can_end_var_search(node: &TreeNode) -> bool {
    if node.is_keyword(FN) {
        return true;
    }
    if node.is_keyword(VAR) {
        return true;
    }
    if let TreeNode::Token(token) = node {
        if token.typ == TokenType::LineComment {
            return true;
        }
    }

    false
}

impl GrammarLike for VarDeclarationGrammar {
    use_parser!(VarDeclarationParser);

    fn next_match_start(&self, nodes: &TreeNodes) -> Option<usize> {
        for (index, node) in nodes.iter().enumerate() {
            let TreeNode::Token(token) = node else {
                continue;
            };
            if &token.string != VAR {
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

            if item.is_token_type(TokenType::Semicolon) {
                return Some(index);
            };

            if can_end_var_search(item) {
                return Some(index - 1);
            }
        }

        Some(start_index)
    }

    fn allow_parallel_processing(&self) -> bool {
        true
    }
}
