use crate::tokens::tokenize::Token;

use super::{document::Document, tree::{TreeNode, Unit, Error}, grammar::{DocumentGrammar, GrammarLike}};

pub fn parse(tokens: &[Token]) -> TreeNode {
    let nodes: Vec<TreeNode> = tokens.iter().map(|token| TreeNode::Token(token.clone())).collect();
    let grammar = DocumentGrammar {};
    if !grammar.has_match(&nodes) {
        return Error::new("Document is empty").into();
    }
    let document = Document::parse(nodes);

    document
}