
use crate::lexer::types::keywords::is_keyword;
use crate::lexer::types::token_type::TokenType;
use crate::parser::nodes::typ::Type;
use crate::parser::tree::{TreeNode, TreeNodeLike, ParseError};

#[derive(Debug, Clone)]
pub struct SimpleType {
    start_index: usize,
    end_index: usize,
    name: String
}

impl SimpleType {
    pub fn parse(node: TreeNode) -> TreeNode {
        let start_index = node.get_start_index();
        let end_index = node.get_end_index();

        let id_error = ParseError::at(start_index, end_index, format!("Expected simple type, got {:?}.",node));

        let TreeNode::Token(token) = node else {
            return id_error.into();
        };
        if token.typ != TokenType::Identifier || is_keyword(&token.string) {
            return id_error.into();
        }

        let name = token.string;

        let typ = SimpleType {
            start_index,
            end_index,
            name
        };

        TreeNode::Type(Type::SimpleType(typ))
    }
}

impl TreeNodeLike for SimpleType {
    fn get_start_index(&self) -> usize {
        self.start_index
    }
    fn get_end_index(&self) -> usize {
        self.end_index
    }
}