use std::collections::VecDeque;


use crate::lexer::types::{token_type::TokenType, keywords::{get_keywords, is_keyword}};

use super::{tree::{TreeNodeLike, TreeNode, ParseError}, grammar::GrammarLike};

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

        let error_end_with_semi = ParseError::from_nodes(&nodes, "Uniform declaration must end with semicolon.");
        let semi_colon = queue.pop_back();
        let Some(semi_colon) = semi_colon else {
            return error_end_with_semi.into();
        };
        let TreeNode::Token(last) = &semi_colon else {
            return error_end_with_semi.into();
        };
        if last.typ != TokenType::Semicolon {
            return error_end_with_semi.into();
        }

        let error_start_with_keyword = ParseError::from_nodes(&nodes,"Uniform declaration must start with keyword 'uniform'.");
        let Some(TreeNode::Token(first)) = queue.pop_front() else {
            return error_start_with_keyword.into();
        };
        if first.typ != TokenType::Identifier || &first.string != UNIFORM_KEYWORD {
            return error_start_with_keyword.into();
        }

        let error_name = ParseError::from_nodes(&nodes,"Uniform name must be an identifier.");
        let name_node = queue.pop_front();
        let Some(name_node) = name_node else {
            return error_name.into();
        };
        let TreeNode::Token(name) = &name_node else {
            return error_name.into();
        };
        let name_text = name.string.clone();
        if name.typ != TokenType::Identifier || is_keyword(&name_text) {
            return error_name.into();
        }

        let error_colon = ParseError::at(
            name_node.get_end_index(),
            name_node.get_end_index()+1,
            "Colon expected after uniform name."
        );

        let colon_node = queue.pop_front();
        let Some(colon_node) = colon_node else {
            return error_colon.into();
        };
        let TreeNode::Token(colon_token) = &colon_node else {
            return error_colon.into();
        };
        if colon_token.typ != TokenType::Colon {
            return error_colon.into();
        }

        let type_nodes: Vec<TreeNode> = queue.into();
        if type_nodes.len() == 0 {
            return ParseError::at(
                colon_node.get_end_index(),
                semi_colon.get_start_index(),
                "Uniform type must not be empty."
            ).into();
        }

        let start_index = colon_token.start_index;
        let end_index = last.end_index;
        let name = colon_token.string.to_owned();

        TreeNode::UniformDeclaration(UniformDeclaration {
            name,
            start_index,
            end_index,
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

fn can_end_uniform_search(node: &TreeNode) -> bool {
    if node.is_token_type(TokenType::Semicolon) {
        return true;
    };
    if node.is_keyword("fn") {
        return true;
    }
    if node.is_keyword("uniform") {
        return true;
    }

    false
}

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

            if can_end_uniform_search(&item) {
                return Some(index)
            }
        }

        None
    }

    fn construct(&self, nodes: Vec<TreeNode>) -> TreeNode {
        UniformDeclaration::parse(nodes)
    }
}