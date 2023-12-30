use std::collections::VecDeque;

use crate::{parser::{tree::{TreeNodeLike, TreeNode, ParseError, get_start_index, get_end_index, get_range}, nodes::function_call_args::FunctionCallArgs, tree_nodes::TreeNodes}, lexer::types::{token_type::TokenType, keywords::is_keyword}, common::range::Range};

use super::expr::{ExpressionLike, Expression};
use crate::pop_front_node;
use crate::pop_back_node;

#[derive(Debug, Clone)]
pub struct FunctionCall {
    name: String,
    range: Range,
    args: Box<TreeNode>
}

impl FunctionCall {
    pub fn parse(mut nodes: TreeNodes) -> TreeNode {
        let range = nodes.range;

        pop_front_node!(
            nodes,
            "Expected identifier at function call.",
            Some(TreeNode::Token(name)),
            name.typ == TokenType::Identifier
            && !is_keyword(&name.string)
        );

        let name = name.string;

        pop_front_node!(
            nodes,
            "Expected opening bracket `(` at function call.",
            Some(TreeNode::Token(opening_bracket)),
            opening_bracket.typ == TokenType::OpeningBracketRound
        );

        pop_back_node!(
            nodes,
            "Mismatched brackets. Expected closing round bracket `()` at function call.",
            Some(TreeNode::Token(opening_bracket)),
            opening_bracket.typ == TokenType::ClosingBracketRound
        );

        let arg_nodes: TreeNodes = nodes;
        let args = FunctionCallArgs::parse(arg_nodes);
        
        let call = FunctionCall {
            name: name,
            range,
            args: Box::new(args)
        };

        TreeNode::Expression(Expression::FunctionCall(call))
    }
}

impl TreeNodeLike for FunctionCall {
    fn get_range(&self) -> Range {
        self.range
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