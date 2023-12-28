use std::collections::VecDeque;


use crate::lexer::types::token_type::TokenType;

use super::{tree::{TreeNodeLike, TreeNode, Error}, grammar::GrammarLike};

#[derive(Debug, Clone)]
pub struct UniformDeclaration {
    pub name: String,
    start_index: usize,
    end_index: usize,
}

impl UniformDeclaration {
    pub fn parse(nodes: Vec<TreeNode>) -> TreeNode {
        let copy = nodes.to_vec();
        let mut queue: VecDeque<TreeNode> = copy.into();

        let error_end_with_semi = Error::from_nodes(&nodes, "Uniform declaration must end with semicolon.");
        let Some(TreeNode::Token(last)) = queue.pop_front() else {
            return error_end_with_semi.into();
        };
        if last.typ != TokenType::Semicolon {
            return error_end_with_semi.into();
        }

        let error_start_with_keyword = Error::from_nodes(&nodes,"Uniform declaration must start with keyword 'uniform'.");
        let Some(TreeNode::Token(first)) = queue.pop_front() else {
            return error_start_with_keyword.into();
        };
        if first.typ != TokenType::Identifier || &first.string != UNIFORM_KEYWORD {
            return error_start_with_keyword.into();
        }

        let error_name = Error::from_nodes(&nodes,"Uniform name must be an identifier");
        let Some(TreeNode::Token(name)) = queue.pop_front() else {
            return error_name.into();
        };
        if name.typ != TokenType::Identifier {
            return error_name.into();
        }

        let start_index = name.start_index;
        let end_index = last.end_index;
        let name = name.string.to_owned();

        TreeNode::UniformDeclaration(UniformDeclaration {
            name,
            start_index,
            end_index
        })
    }
}

impl TreeNodeLike for UniformDeclaration {
    fn get_start_index(&self) -> usize {
        self.start_index
    }
    fn get_end_index(&self) -> usize {
        self.end_index
    }
}

pub struct UniformGrammar {}

static UNIFORM_KEYWORD: &'static str = "uniform";

impl GrammarLike for UniformGrammar {
    fn next_match_at(&self, nodes: &[TreeNode]) -> Option<usize> {
        for (index, node) in nodes.iter().enumerate() {
            let TreeNode::Token(token) = node else {
                continue
            };
            if &token.string != UNIFORM_KEYWORD {
                continue;
            }
            return Some(index);
        }

        None
    }

    fn find_match_end(&self, nodes: &[TreeNode], start_index: usize) -> Option<usize> {
        for (index, item) in nodes.iter().enumerate() {
            if index <= start_index {
                continue;
            }
            let TreeNode::Token(token) = item else {
                continue;
            };
            if token.typ != TokenType::Semicolon {
                continue;
            }
            return Some(index)
        }

        None
    }

    fn construct(&self, nodes: Vec<TreeNode>) -> TreeNode {
        UniformDeclaration::parse(nodes)
    }
}