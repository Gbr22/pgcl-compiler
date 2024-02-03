use crate::{
    lexer::types::{keywords::is_keyword, token_type::TokenType},
    parser::{
        brackets::round_bracket, grammar::GrammarLike, match_brackets::find_bracket_end,
        parse::Parser, parsers::expressions::function_call::FunctionCallParser, tree::TreeNode,
        tree_nodes::TreeNodes,
    },
    use_parser,
};

pub struct FunctionCallGrammar {}

fn is_function_name(node: &TreeNode) -> bool {
    let TreeNode::Token(token) = node else {
        return false;
    };
    if token.typ != TokenType::Identifier {
        return false;
    }
    if is_keyword(&token.string) {
        return false;
    }

    true
}

impl GrammarLike for FunctionCallGrammar {
    use_parser!(FunctionCallParser);

    fn next_match_start(&self, nodes: &TreeNodes) -> Option<usize> {
        let mut is_last_node_function_name = false;
        for (index, node) in nodes.iter().enumerate() {
            if node.is_token_type(TokenType::OpeningBracketRound) && is_last_node_function_name {
                return Some(index - 1); // start at prev node (function name)
            }

            is_last_node_function_name = is_function_name(node);
        }

        None
    }
    fn next_match_end(&self, nodes: &TreeNodes, start_index: usize) -> Option<usize> {
        find_bracket_end(
            round_bracket(),
            start_index + 1, // start_index is the index of the identifier
            nodes.iter(),
        )
    }
    fn allow_parallel_processing(&self) -> bool {
        true
    }
}
