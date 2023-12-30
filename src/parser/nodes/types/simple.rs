
use crate::common::range::Range;
use crate::lexer::types::keywords::is_keyword;
use crate::lexer::types::token_type::TokenType;
use crate::parser::tree::{TreeNode, TreeNodeLike, ParseError};
use super::typ::{Type, TypeLike};

#[derive(Debug, Clone)]
pub struct SimpleType {
    range: Range,
    name: String
}

impl SimpleType {
    pub fn parse(node: TreeNode) -> TreeNode {
        let range = node.get_range();

        let id_error = ParseError::at(range, format!("Expected simple type, got {:?}.",node));

        let TreeNode::Token(token) = node else {
            return id_error.into();
        };
        if token.typ != TokenType::Identifier || is_keyword(&token.string) {
            return id_error.into();
        }

        let name = token.string;

        let typ = SimpleType {
            name,
            range,
        };

        TreeNode::Type(Type::SimpleType(typ))
    }
}


impl TreeNodeLike for SimpleType {
    fn get_range(&self) -> Range {
        self.range
    }
}

impl TypeLike for SimpleType {
    fn to_node_like(&self) -> Box<&dyn TreeNodeLike> {
        Box::new(self)
    }
}