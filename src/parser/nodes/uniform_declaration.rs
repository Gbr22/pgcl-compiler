use std::collections::VecDeque;
use crate::pop_back_node;
use crate::pop_front_node;

use crate::{lexer::types::{token_type::TokenType, keywords::{is_keyword, UNIFORM}}, parser::{tree::{TreeNode, ParseError, TreeNodeLike, get_range}, tree_nodes::TreeNodes}, common::range::{Range, Len}};

use super::types::typ::Type;

#[derive(Debug, Clone)]
pub struct UniformDeclaration {
    pub name: String,
    pub typ: Box<TreeNode>,
    range: Range
}

impl UniformDeclaration {
    pub fn parse(mut nodes: TreeNodes) -> TreeNode {
        let range = nodes.range;

        pop_back_node!(
            nodes,
            "Uniform declaration must end with semicolon.",
            Some(TreeNode::Token(semi_colon)),
            semi_colon.typ == TokenType::Semicolon
        );

        pop_front_node!(
            nodes,
            "Uniform declaration must start with keyword 'uniform'.",
            Some(TreeNode::Token(uniform_keyword)),
            uniform_keyword.typ == TokenType::Identifier
            && uniform_keyword.string == UNIFORM
        );

        pop_front_node!(
            nodes,
            "Uniform name must be an identifier.",
            Some(TreeNode::Token(name)),
            name.typ == TokenType::Identifier
            && !is_keyword(&name.string)
        );

        let name = name.string;

        pop_front_node!(
            nodes,
            "Colon expected after uniform name.",
            Some(TreeNode::Token(colon)),
            colon.typ == TokenType::Colon
        );

        let type_nodes = nodes;
        if type_nodes.len() == 0 {
            return ParseError::at(
                Range::between(colon.get_range(), semi_colon.get_range()),
                format!("Uniform type must not be empty.")
            ).into();
        }

        let typ = Type::parse(type_nodes);

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
