use std::collections::VecDeque;


use crate::{lexer::types::{token_type::TokenType, keywords::{is_keyword, UNIFORM}}, parser::tree::{TreeNode, ParseError, TreeNodeLike}};

use super::types::typ::Type;

#[derive(Debug, Clone)]
pub struct UniformDeclaration {
    pub name: String,
    start_index: usize,
    end_index: usize,
    pub typ: Box<TreeNode>
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
        if first.typ != TokenType::Identifier || &first.string != UNIFORM {
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

        let typ = Type::parse(type_nodes);

        let start_index = colon_token.range.start_index;
        let end_index = last.range.end_index;
        let name = colon_token.string.to_owned();

        TreeNode::UniformDeclaration(UniformDeclaration {
            name,
            start_index,
            end_index,
            typ: Box::new(typ)
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
    fn get_errors(&self) -> Vec<ParseError> {
        let typ: TreeNode = *self.typ.clone();
        let TreeNode::ParseError(error) = typ else {
            return vec![];
        };

        return vec![error];
    }
}
