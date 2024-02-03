use crate::pop_back_node;
use crate::pop_front_node;
use std::collections::VecDeque;

use crate::{
    common::range::{Len, Range},
    lexer::types::{
        keywords::{is_keyword, UNIFORM},
        token_type::TokenType,
    },
    parser::{
        tree::{get_range, ParseError, TreeNode, TreeNodeLike},
        tree_nodes::TreeNodes,
    },
};

use super::types::typ::Type;

#[derive(Debug, Clone)]
pub struct UniformDeclaration {
    pub name: String,
    pub typ: Box<TreeNode>,
    pub range: Range,
}

impl UniformDeclaration {}

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
