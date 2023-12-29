use std::collections::VecDeque;


use crate::{lexer::types::{token_type::TokenType, keywords::{is_keyword, UNIFORM, FN}}, parser::{tree::{TreeNode, ParseError, TreeNodeLike, get_start_index, get_end_index}, grammars::function_declaration::{find_args_end, find_body_start, find_body_end}}};

use super::{typ::Type, block::Block};

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    name: String,
    start_index: usize,
    end_index: usize,
    return_type: Box<TreeNode>,
    body: Box<TreeNode>
}

impl FunctionDeclaration {
    pub fn parse(original_nodes: Vec<TreeNode>) -> TreeNode {
        let start_index = get_start_index(&original_nodes).unwrap_or_default();
        let end_index = get_end_index(&original_nodes).unwrap_or_default();
        let mut queue: VecDeque<TreeNode> = original_nodes.clone().into();

        let missing_keyword = ParseError::from_nodes(&original_nodes, format!("Missing fn keyword."));
        let Some(fn_keyword) = queue.pop_front() else {
            return missing_keyword.into();
        };
        if !fn_keyword.is_keyword(FN) {
            return missing_keyword.into();
        }
        let name_error = ParseError::from_nodes(&original_nodes, format!("Missing function name."));
        let Some(TreeNode::Token(name_node)) = queue.pop_front() else {
            return name_error.into();
        };
        if name_node.typ != TokenType::Identifier || is_keyword(&name_node.string) {
            return name_error.into();
        }
        let name = name_node.string;
        let expected_args = ParseError::from_nodes(&original_nodes,format!("Expected opening round bracket after function name."));
        let Some(args_open) = queue.pop_front() else {
            return expected_args.into();
        };
        if !args_open.is_token_type(TokenType::OpeningBracketRound) {
            return expected_args.into();
        }
        let mut nodes: Vec<TreeNode> = queue.into();
        let Some(args_end_index) = find_args_end(0, &nodes) else {
            return ParseError::from_nodes(&original_nodes,format!("Mismatched brackets. Round bracket `()` not closed.")).into();
        };
        let argument_nodes: Vec<TreeNode> = nodes.splice(0..args_end_index, vec![]).collect();
        let mut queue: VecDeque<TreeNode> = nodes.into();
        queue.pop_front(); // remove closing bracket `)`

        // TODO parse arguments

        let expected_arrow = ParseError::from_nodes(&original_nodes, format!("Expected `->` after function parameters."));
        let Some(arrow) = queue.pop_front() else {
            return expected_arrow.into();
        };
        if !arrow.is_token_type(TokenType::ArrowRight) {
            return expected_arrow.into();
        }

        let mut nodes: Vec<TreeNode> = queue.into();
        let Some(body_start_index) = find_body_start(0, &nodes) else {
            return ParseError::from_nodes(&original_nodes, format!("Could not find start of function body.")).into();
        };
        let Some(body_end_index) = find_body_end(body_start_index, &nodes) else {
            return ParseError::from_nodes(&original_nodes, format!("Mismatched brackets. Curly bracket `{{}}` not closed.")).into();
        };
        let after_body: Vec<TreeNode> = nodes.splice(body_end_index.., vec![]).collect();
        // there should only be a single `}` in the vector
        if after_body.len() > 1 {
            return ParseError::from_nodes(&original_nodes, format!("Unexpected items after function body.")).into();
        }
        let body_nodes: Vec<TreeNode> = nodes.splice(body_start_index..body_end_index, vec![]).collect();
        let body_block = Block::parse(body_nodes);

        let type_nodes = nodes;
        let typ = Type::parse(type_nodes);

        TreeNode::FunctionDeclaration(FunctionDeclaration {
            start_index,
            end_index,
            name,
            return_type: Box::new(typ),
            body: Box::new(body_block)
        })
    }
}

impl TreeNodeLike for FunctionDeclaration {
    fn get_start_index(&self) -> usize {
        self.start_index
    }
    fn get_end_index(&self) -> usize {
        self.end_index
    }
    fn get_errors(&self) -> Vec<ParseError> {
        let mut errors: Vec<ParseError> = vec![];
        errors.extend(self.return_type.get_errors());

        errors
    }
}
