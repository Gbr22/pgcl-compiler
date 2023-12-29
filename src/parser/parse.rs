
use crate::lexer::token::Token;

use super::{document::Document, tree::{TreeNode, ParseError}, grammar::{DocumentGrammar, GrammarLike}};

pub fn parse(tokens: &[Token]) -> TreeNode {
    let nodes: Vec<TreeNode> = tokens.iter().map(|token| TreeNode::Token(token.clone())).collect();
    let grammar = DocumentGrammar {};
    if !grammar.has_match(&nodes) {
        return ParseError::new("Document is empty").into();
    }
    let document = Document::parse(nodes);

    document
}