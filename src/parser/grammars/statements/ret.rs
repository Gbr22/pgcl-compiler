use crate::{
    lexer::types::{keywords::RETURN, token_type::TokenType},
    parser::{
        grammar::GrammarLike, parse::Parser, parsers::statements::ret::ReturnStatementParser,
        tree::TreeNode, tree_nodes::TreeNodes,
    },
    use_parser,
};

pub struct ReturnStatementGrammar {}

impl GrammarLike for ReturnStatementGrammar {
    use_parser!(ReturnStatementParser);

    fn next_match_start(&self, nodes: &TreeNodes) -> Option<usize> {
        for (index, node) in nodes.iter().enumerate() {
            if node.is_keyword(RETURN) {
                return Some(index);
            }
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
