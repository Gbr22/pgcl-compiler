use std::collections::VecDeque;


use crate::{lexer::types::{token_type::TokenType, keywords::{is_keyword, UNIFORM, FN}}, parser::{tree::{TreeNode, ParseError, TreeNodeLike, get_start_index, get_end_index, get_range}, grammars::function_declaration::{find_args_end, find_body_start, find_body_end}, tree_nodes::TreeNodes}, common::range::Range};

use super::{block::Block, types::typ::Type};


#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    name: String,
    return_type: Box<TreeNode>,
    body: Box<TreeNode>,
    range: Range
}

impl FunctionDeclaration {
    pub fn parse(mut nodes: TreeNodes) -> TreeNode {
        let range = nodes.range;

        let missing_keyword = ParseError::at(range, format!("Missing fn keyword."));
        let Some(fn_keyword) = nodes.pop_front() else {
            return missing_keyword.into();
        };
        if !fn_keyword.is_keyword(FN) {
            return missing_keyword.into();
        }
        let name_error = ParseError::at(range, format!("Missing function name."));
        let Some(TreeNode::Token(name_node)) = nodes.pop_front() else {
            return name_error.into();
        };
        if name_node.typ != TokenType::Identifier || is_keyword(&name_node.string) {
            return name_error.into();
        }
        let name = name_node.string;
        let expected_args = ParseError::at(range,format!("Expected opening round bracket after function name."));
        let Some(args_open) = nodes.pop_front() else {
            return expected_args.into();
        };
        if !args_open.is_token_type(TokenType::OpeningBracketRound) {
            return expected_args.into();
        }
        let Some(args_end_index) = find_args_end(0, nodes.iter()) else {
            return ParseError::at(range,format!("Mismatched brackets. Round bracket `()` not closed.")).into();
        };
        let argument_nodes = nodes.slice(0, args_end_index);
        nodes.pop_front(); // remove closing bracket `)`

        // TODO parse arguments

        let expected_arrow = ParseError::at(range, format!("Expected `->` after function parameters."));
        let Some(arrow) = nodes.pop_front() else {
            return expected_arrow.into();
        };
        if !arrow.is_token_type(TokenType::ArrowRight) {
            return expected_arrow.into();
        }

        let Some(body_start_index) = find_body_start(0, nodes.iter()) else {
            return ParseError::at(range, format!("Could not find start of function body.")).into();
        };
        let Some(body_end_index) = find_body_end(body_start_index, nodes.iter()) else {
            return ParseError::at(range, format!("Mismatched brackets. Curly bracket `{{}}` not closed.")).into();
        };
        let after_body = nodes.slice(body_end_index,usize::MAX);
        // there should only be a single `}` in the vector
        if after_body.len() > 1 {
            return ParseError::at(range, format!("Unexpected items after function body.")).into();
        }
        let mut body_nodes = nodes.slice(body_start_index,body_end_index);
        body_nodes.pop_front(); // skip opening curly `{`

        let body_block = Block::parse(body_nodes);

        let type_nodes = nodes;
        let typ = Type::parse(type_nodes);

        TreeNode::FunctionDeclaration(FunctionDeclaration {
            name,
            return_type: Box::new(typ),
            body: Box::new(body_block),
            range
        })
    }
}

impl TreeNodeLike for FunctionDeclaration {
    fn get_range(&self) -> Range {
        self.range
    }
    fn get_errors(&self) -> Vec<ParseError> {
        let mut errors: Vec<ParseError> = vec![];
        errors.extend(self.return_type.get_errors());
        errors.extend(self.body.get_errors());

        errors
    }
}
