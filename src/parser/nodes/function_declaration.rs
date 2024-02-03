use super::{block::Block, types::typ::Type};
use crate::pop_front_node;
use crate::{
    common::range::Range,
    lexer::types::{
        keywords::{is_keyword, FN, UNIFORM},
        token_type::TokenType,
    },
    parser::{
        grammars::function_declaration::{find_args_end, find_body_end, find_body_start},
        tree::{get_end_index, get_range, get_start_index, ParseError, TreeNode, TreeNodeLike},
        tree_nodes::TreeNodes,
    },
};

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    pub name: String,
    pub return_type: Box<TreeNode>,
    pub body: Box<TreeNode>,
    pub range: Range,
}

impl FunctionDeclaration {}

impl TreeNodeLike for FunctionDeclaration {
    fn get_range(&self) -> Range {
        self.range
    }
    fn get_errors(&self) -> Vec<ParseError> {
        let mut errors: Vec<ParseError> = vec![];
        errors.extend(self.return_type.get_errors());
        errors.extend(self.body.get_errors());

        errors
    }
}
