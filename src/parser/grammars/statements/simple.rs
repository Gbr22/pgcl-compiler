use crate::{
    lexer::types::token_type::TokenType,
    parser::{
        grammar::GrammarLike, parse::Parser, parsers::statements::simple::SimpleStatementParser,
        tree::TreeNode, tree_nodes::TreeNodes,
    },
    use_parser,
};

pub struct SimpleStatementGrammar {}

impl GrammarLike for SimpleStatementGrammar {
    use_parser!(SimpleStatementParser);

    fn next_match_start(&self, nodes: &TreeNodes) -> Option<usize> {
        if nodes.len() == 0 {
            return None;
        }
        for (index, node) in nodes.iter().enumerate() {
            if let TreeNode::Statement(_) = node {
                continue;
            }
            if let TreeNode::VarDeclaration(_) = node {
                continue;
            }
            if let TreeNode::ParseError(_) = node {
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
    fn allow_parallel_processing(&self) -> bool {
        true
    }
}
