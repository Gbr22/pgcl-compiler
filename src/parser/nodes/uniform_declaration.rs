use std::collections::VecDeque;


use crate::{lexer::types::{token_type::TokenType, keywords::{is_keyword, UNIFORM}}, parser::{tree::{TreeNode, ParseError, TreeNodeLike, get_range}, tree_nodes::TreeNodes}, common::range::{Range, Len}};

use super::types::typ::Type;

#[derive(Debug, Clone)]
pub struct UniformDeclaration {
    pub name: String,
    pub typ: Box<TreeNode>,
    range: Range
}

impl UniformDeclaration {
    pub fn parse(nodes: TreeNodes) -> TreeNode {
        let range = nodes.range;
        let nodes = nodes.vec;
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
            name_node.get_range() + Len(1),
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
                Range::between(colon_node.get_range(), semi_colon.get_range()),
                format!("Uniform type must not be empty.")
            ).into();
        }

        let type_nodes = TreeNodes::new(range, type_nodes);
        let typ = Type::parse(type_nodes);

        let name = colon_token.string.to_owned();
        TreeNode::UniformDeclaration(UniformDeclaration {
            name,
            range,
            typ: Box::new(typ)
        })
    }
}

impl TreeNodeLike for UniformDeclaration {
    fn get_range(&self) -> Range {
        self.range
    }
    fn get_errors(&self) -> Vec<ParseError> {
        let typ: TreeNode = *self.typ.clone();
        let TreeNode::ParseError(error) = typ else {
            return vec![];
        };

        return vec![error];
    }
}
