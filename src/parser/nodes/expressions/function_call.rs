use std::collections::VecDeque;

use crate::{parser::{tree::{TreeNodeLike, TreeNode, ParseError, get_start_index, get_end_index}, nodes::function_call_args::FunctionCallArgs}, lexer::types::{token_type::TokenType, keywords::is_keyword}};

use super::expr::{ExpressionLike, Expression};

#[derive(Debug, Clone)]
pub struct FunctionCall {
    name: String,
    start_index: usize,
    end_index: usize,
    args: Box<TreeNode>
}

impl FunctionCall {
    pub fn parse(nodes: Vec<TreeNode>) -> TreeNode {
        let start_index = get_start_index(&nodes).unwrap_or_default();
        let end_index = get_end_index(&nodes).unwrap_or_default();
        
        let mut queue: VecDeque<TreeNode> = nodes.into();
        let name_node = queue.pop_front();
        let name_error = ParseError::at(start_index,end_index,format!("Expected identifier at function call."));
        let Some(TreeNode::Token(name_token)) = name_node else {
            return name_error.into();
        };
        if name_token.typ != TokenType::Identifier || is_keyword(&name_token.string) {
            return name_error.into();
        }
        let name = name_token.string;

        let missing_opening = ParseError::at(start_index, end_index, format!("Expected opening bracket `(` at function call."));
        let Some(opening_bracket) = queue.pop_front() else {
            return missing_opening.into();
        };
        if !opening_bracket.is_token_type(TokenType::OpeningBracketRound) {
            return missing_opening.into(); 
        }
        
        let missing_closing = ParseError::at(start_index, end_index, format!("Mismatched brackets. Expected closing round bracket `()` at function call. {:#?}",&queue));
        let Some(closing_bracket) = queue.pop_back() else {
            return missing_closing.into();
        };
        if !closing_bracket.is_token_type(TokenType::ClosingBracketRound) {
            return missing_closing.into(); 
        }

        let arg_nodes: Vec<TreeNode> = queue.into();
        let args = FunctionCallArgs::parse(arg_nodes);
        
        let call = FunctionCall {
            name: name,
            start_index,
            end_index,
            args: Box::new(args)
        };

        TreeNode::Expression(Expression::FunctionCall(call))
    }
}

impl TreeNodeLike for FunctionCall {
    fn get_start_index(&self) -> usize {
        self.start_index
    }

    fn get_end_index(&self) -> usize {
        self.end_index
    }

    fn get_errors(&self) -> Vec<ParseError> {
        self.args.get_errors()
    }
}

impl ExpressionLike for FunctionCall {
    fn to_node_like(&self) -> Box<&dyn TreeNodeLike> {
        Box::new(self)
    }
}