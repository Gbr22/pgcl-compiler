use std::collections::VecDeque;


use crate::{lexer::types::{token_type::TokenType, keywords::{is_keyword, UNIFORM, FN}}, parser::{tree::{TreeNode, ParseError, TreeNodeLike, get_start_index, get_end_index}, grammars::{function_declaration::{find_args_end, find_body_start, find_body_end}, function_call_arg::FunctionCallArgGrammar}, grammar::GrammarLike}};

use super::{block::Block, types::typ::Type};


#[derive(Debug, Clone)]
pub struct FunctionCallArgs {
    start_index: usize,
    end_index: usize,
    args: Vec<TreeNode>
}

impl FunctionCallArgs {
    pub fn parse(nodes: Vec<TreeNode>) -> TreeNode {
        let start_index = get_start_index(&nodes).unwrap_or_default();
        let end_index = get_end_index(&nodes).unwrap_or_default();

        let function_call_arg_grammar = FunctionCallArgGrammar {};
        let nodes = function_call_arg_grammar.process_all(nodes);

        let nodes: Vec<TreeNode> = nodes.into_iter().map(|node|{
            /* if let TreeNode::ParseError(error) = node {
                return TreeNode::ParseError(error);
            }
            if let TreeNode::Expression(expr) = node {
                return TreeNode::Expression(expr);
            } */
            
            return node;
            //return ParseError::from_nodes(&vec![node], format!("Function calls may only contain expressions.")).into();
        }).collect();

        let fn_call_args = FunctionCallArgs {
            start_index,
            end_index,
            args: nodes
        };
        TreeNode::FunctionCallArgs(fn_call_args)
    }
}

impl TreeNodeLike for FunctionCallArgs {
    fn get_start_index(&self) -> usize {
        self.start_index
    }
    fn get_end_index(&self) -> usize {
        self.end_index
    }
    fn get_errors(&self) -> Vec<ParseError> {
        let errors: Vec<ParseError> = self.args.iter()
            .flat_map(|arg|arg.get_errors())
            .collect();

        errors
    }
}
