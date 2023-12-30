use crate::{parser::tree::{TreeNodeLike, TreeNode, ParseError}, lexer::types::token_type::TokenType};

use super::expr::{ExpressionLike, Expression};

#[derive(Debug, Clone)]
pub struct ValueAccess {
    name: String,
    start_index: usize,
    end_index: usize
}

impl ValueAccess {
    pub fn parse(node: TreeNode) -> TreeNode {
        let start_index = node.get_start_index();
        let end_index = node.get_end_index();
        let id_error = ParseError::at(start_index, end_index, format!("Expected identifier."));
        let TreeNode::Token(name) = node else {
            return id_error.into();
        };
        if name.typ != TokenType::Identifier {
            return id_error.into();
        }
        let name = name.string.to_owned();

        let value_access = ValueAccess {
            name,
            start_index,
            end_index
        };
        TreeNode::Expression(Expression::ValueAccess(value_access))
    }
}

impl TreeNodeLike for ValueAccess {
    fn get_start_index(&self) -> usize {
        self.start_index
    }

    fn get_end_index(&self) -> usize {
        self.end_index
    }
}

impl ExpressionLike for ValueAccess {
    fn to_node_like(&self) -> Box<&dyn TreeNodeLike> {
        Box::new(self)
    }
}