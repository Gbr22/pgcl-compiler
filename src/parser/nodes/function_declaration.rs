use std::collections::VecDeque;


use crate::{lexer::types::{token_type::TokenType, keywords::{is_keyword, UNIFORM}}, parser::tree::{TreeNode, ParseError, TreeNodeLike, get_start_index, get_end_index}};

use super::typ::Type;

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    start_index: usize,
    end_index: usize,
}

impl FunctionDeclaration {
    pub fn parse(nodes: Vec<TreeNode>) -> TreeNode {
        let start_index = get_start_index(&nodes).unwrap_or_default();
        let end_index = get_end_index(&nodes).unwrap_or_default();

        return ParseError::from_nodes(&nodes, format!("Test")).into();

        TreeNode::FunctionDeclaration(FunctionDeclaration {
            start_index,
            end_index,
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
}
