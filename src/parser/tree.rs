use crate::parser::document::Document;
use crate::parser::uniform::UniformDeclaration;
use crate::lexer::token::Token;

trait_enum!{
    #[derive(Debug, Clone)]
    pub enum TreeNode: TreeNodeLike {
        Token,
        Unit,
        Document,
        UniformDeclaration,
        Error
    }
}

pub trait TreeNodeLike {
    fn get_start_index(&self) -> usize;
    fn get_end_index(&self) -> usize;
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
pub struct Error {
    pub text: String,
    start_index: usize,
    end_index: usize
}

impl Error {
    pub fn new(text: impl Into<String>) -> Error {
        Error { text: text.into(), start_index: 0, end_index: 0 }
    }
    pub fn from_nodes(nodes: &[TreeNode], text: impl Into<String>) -> Error {
        let text = text.into();
        let start_index = nodes
            .first()
            .map(|node|node.get_start_index())
            .unwrap_or_default();
        let end_index = nodes
            .last()
            .map(|node|node.get_end_index())
            .unwrap_or_default();

        Error {
            text,
            start_index,
            end_index
        }
    }
}

impl Into<TreeNode> for Error {
    fn into(self) -> TreeNode {
        TreeNode::Error(self)
    }
}

impl TreeNodeLike for Error {
    fn get_start_index(&self) -> usize {
        self.start_index
    }

    fn get_end_index(&self) -> usize {
        self.end_index
    }
}