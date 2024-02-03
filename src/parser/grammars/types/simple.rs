use crate::{
    lexer::types::{keywords::is_keyword, token_type::TokenType},
    parser::{
        grammar::GrammarLike, parse::Parser, parsers::types::simple::SimpleTypeParser,
        tree::TreeNode, tree_nodes::TreeNodes,
    },
    use_parser,
};

pub struct SimpleTypeGrammar {}

impl GrammarLike for SimpleTypeGrammar {
    use_parser!(SimpleTypeParser);

    fn next_match_start(&self, nodes: &TreeNodes) -> Option<usize> {
        for (index, node) in nodes.iter().enumerate() {
            let TreeNode::Token(token) = node else {
                continue;
            };
            if token.typ != TokenType::Identifier || is_keyword(&token.string) {
                continue;
            }

            return Some(index);
        }

        None
    }
    fn next_match_end(&self, _nodes: &TreeNodes, start_index: usize) -> Option<usize> {
        Some(start_index)
    }
}
