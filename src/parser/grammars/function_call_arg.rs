use crate::lexer::types::keywords::FN;
use crate::parser::brackets::{round_bracket, curly_bracket};
use crate::parser::match_brackets::find_bracket_end;
use crate::parser::nodes::comma_separator::find_next_comma_outside_brackets;
use crate::parser::nodes::function_call_arg::FunctionCallArg;
use crate::{parser::{tree::TreeNode, grammar::GrammarLike}, lexer::types::{token_type::TokenType, keywords::UNIFORM}};

pub struct FunctionCallArgGrammar {}

impl GrammarLike for FunctionCallArgGrammar {
    fn next_match_start(&self, nodes: &[TreeNode]) -> Option<usize> {
        if nodes.len() == 0 {
            return None
        }

        for (index, node) in nodes.iter().enumerate() {
            if let TreeNode::FunctionCallArg(_) = node {
                continue;
            }

            return Some(index);
        }

        None
    }

    fn next_match_end(&self, nodes: &[TreeNode], start_index: usize) -> Option<usize> {
        let index = find_next_comma_outside_brackets(start_index, nodes);
        
        match index {
            Some(index)=>Some(index),
            None=>Some(nodes.len()-1)
        }
    }

    fn construct(&self, nodes: Vec<TreeNode>) -> TreeNode {
        FunctionCallArg::parse(nodes)
    }
}