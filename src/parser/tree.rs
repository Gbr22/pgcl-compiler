use crate::lexer::types::token_type::TokenType;
use crate::lexer::token::Token;
use crate::parser::nodes::document::Document;
use crate::parser::nodes::uniform_declaration::UniformDeclaration;
use crate::parser::nodes::function_declaration::FunctionDeclaration;
use crate::parser::nodes::block::Block;
use crate::parser::nodes::statements::statement::Statement;
use crate::parser::nodes::types::typ::Type;

trait_enum!{
    #[derive(Debug, Clone)]
    pub enum TreeNode: TreeNodeLike {
        Token,
        Unit,
        Document,
        UniformDeclaration,
        ParseError,
        Type,
        FunctionDeclaration,
        Block,
        Statement,
    }
}

impl TreeNode {
    pub fn is_token_type(&self, typ: TokenType) -> bool {
        let TreeNode::Token(token) = self else {
            return false;
        };

        token.typ == typ
    }
    pub fn is_keyword(&self, str: &str) -> bool {
        let TreeNode::Token(token) = self else {
            return false;
        };

        token.typ == TokenType::Identifier && token.string == str
    }
    pub fn is_error(&self) -> bool {
        if let TreeNode::ParseError(_) = self {
            true
        } else {
            false
        }
    }
}

pub trait TreeNodeLike {
    fn get_start_index(&self) -> usize;
    fn get_end_index(&self) -> usize;
    fn get_errors(&self) -> Vec<ParseError> {
        return vec![];
    }
}

impl TreeNodeLike for Token {
    fn get_start_index(&self) -> usize {
        self.start_index
    }
    fn get_end_index(&self) -> usize {
        self.end_index
    }
}

#[derive(Debug, Clone)]
pub struct Unit {
    start_index: usize,
}
impl Unit {
    pub fn new(pos: usize) -> Self {
        Unit {
            start_index: pos
        }
    }
}
impl TreeNodeLike for Unit {
    fn get_start_index(&self) -> usize {
        self.start_index
    }
    fn get_end_index(&self) -> usize {
        self.start_index
    }
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub text: String,
    start_index: usize,
    end_index: usize
}

impl ParseError {
    pub fn new(text: impl Into<String>) -> ParseError {
        ParseError { text: text.into(), start_index: 0, end_index: 0 }
    }
    pub fn at(start_index: usize, end_index: usize, text: impl Into<String>) -> ParseError {
        ParseError {
            start_index,
            end_index,
            text: text.into()
        }
    }
    pub fn from_nodes(nodes: &[TreeNode], text: impl Into<String>) -> ParseError {
        let text = text.into();
        let start_index = nodes
            .first()
            .map(|node|node.get_start_index())
            .unwrap_or_default();
        let end_index = nodes
            .last()
            .map(|node|node.get_end_index())
            .unwrap_or_default();

        ParseError {
            text,
            start_index,
            end_index
        }
    }
}

impl Into<TreeNode> for ParseError {
    fn into(self) -> TreeNode {
        TreeNode::ParseError(self)
    }
}

impl TreeNodeLike for ParseError {
    fn get_start_index(&self) -> usize {
        self.start_index
    }

    fn get_end_index(&self) -> usize {
        self.end_index
    }

    fn get_errors(&self) -> Vec<ParseError> {
        vec![self.to_owned()]
    }
}

pub fn get_start_index(nodes: &[TreeNode]) -> Option<usize> {
    nodes.first().map(|f|f.get_start_index())
}
pub fn get_end_index(nodes: &[TreeNode]) -> Option<usize> {
    nodes.last().map(|f|f.get_end_index())
}