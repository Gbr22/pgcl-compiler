use crate::{parser::tree::{TreeNodeLike, TreeNode, ParseError, get_range}, lexer::types::token_type::TokenType, common::range::Range};

use super::expr::{ExpressionLike, Expression};

#[derive(Debug, Clone)]
pub struct ValueAccess {
    name: String,
    range: Range,
}

impl ValueAccess {
    pub fn parse(node: TreeNode) -> TreeNode {
        let range = node.get_range();

        let id_error = ParseError::at(range, format!("Expected identifier."));
        let TreeNode::Token(name) = node else {
            return id_error.into();
        };
        if name.typ != TokenType::Identifier {
            return id_error.into();
        }
        let name = name.string.to_owned();

        let value_access = ValueAccess {
            name,
            range
        };
        TreeNode::Expression(Expression::ValueAccess(value_access))
    }
}

impl TreeNodeLike for ValueAccess {
    fn get_range(&self) -> Range {
        self.range
    }
}

impl ExpressionLike for ValueAccess {
    fn to_node_like(&self) -> Box<&dyn TreeNodeLike> {
        Box::new(self)
    }
}