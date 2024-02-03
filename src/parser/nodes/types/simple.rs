use super::typ::{Type, TypeLike};
use crate::common::range::Range;
use crate::lexer::types::keywords::is_keyword;
use crate::lexer::types::token_type::TokenType;
use crate::parser::tree::{ParseError, TreeNode, TreeNodeLike};

#[derive(Debug, Clone)]
pub struct SimpleType {
    pub range: Range,
    pub name: String,
}

impl SimpleType {}

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
