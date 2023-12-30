use crate::{parser::{grammar::GrammarLike, tree::TreeNode, nodes::{document::Document, statements::{simple_statement::SimpleStatement, return_statement::ReturnStatement}, expressions::function_call::FunctionCall}, match_brackets::find_bracket_end}, lexer::types::{token_type::TokenType, keywords::{RETURN, is_keyword}, brackets::{is_opening_bracket_round, is_closing_bracket_round}}};

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
    fn next_match_at(&self, nodes: &[TreeNode]) -> Option<usize> {
        let mut is_last_node_function_name = false;
        for (index, node) in nodes.iter().enumerate() {
            if node.is_token_type(TokenType::OpeningBracketRound) && is_last_node_function_name {
                return Some(index-1); // start at prev node (function name)
            }

            is_last_node_function_name = is_function_name(node);
        }

        None
    }
    fn find_match_end(&self, nodes: &[TreeNode], start_index: usize) -> Option<usize> {
        find_bracket_end(
            is_opening_bracket_round,
            is_closing_bracket_round,
            start_index+1, // start_index is the index of the identifier
            nodes
        )
    }
    fn construct(&self, nodes: Vec<TreeNode>) -> TreeNode {
        let statement = FunctionCall::parse(nodes);

        statement
    }
}