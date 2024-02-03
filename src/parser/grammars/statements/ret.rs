use crate::{
    lexer::types::keywords::RETURN,
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
    fn next_match_end(&self, nodes: &TreeNodes, _start_index: usize) -> Option<usize> {
        Some(nodes.len() - 1)
    }

    fn allow_parallel_processing(&self) -> bool {
        true
    }
}
