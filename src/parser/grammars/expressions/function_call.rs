use crate::{parser::{grammar::GrammarLike, tree::TreeNode, nodes::{document::Document, statements::{simple_statement::SimpleStatement, ret::ReturnStatement}, expressions::function_call::FunctionCall}, match_brackets::find_bracket_end, brackets::round_bracket, tree_nodes::TreeNodes}, lexer::types::{token_type::TokenType, keywords::{RETURN, is_keyword}}};

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
    fn next_match_start(&self, nodes: &TreeNodes) -> Option<usize> {
        let mut is_last_node_function_name = false;
        for (index, node) in nodes.iter().enumerate() {
            if node.is_token_type(TokenType::OpeningBracketRound) && is_last_node_function_name {
                return Some(index-1); // start at prev node (function name)
            }

            is_last_node_function_name = is_function_name(node);
        }

        None
    }
    fn next_match_end(&self, nodes: &TreeNodes, start_index: usize) -> Option<usize> {
        find_bracket_end(
            round_bracket(),
            start_index+1, // start_index is the index of the identifier
            nodes.iter()
        )
    }
    fn construct(&self, nodes: TreeNodes) -> TreeNode {
        let statement = FunctionCall::parse(nodes);

        statement
    }
}