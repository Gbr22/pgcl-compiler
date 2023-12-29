
use crate::lexer::token::Token;

use super::{tree::{TreeNode, ParseError}, grammars::document::DocumentGrammar, grammar::GrammarLike, nodes::document::Document};

pub fn parse(tokens: &[Token]) -> TreeNode {
    let nodes: Vec<TreeNode> = tokens.iter().map(|token| TreeNode::Token(token.clone())).collect();
    let grammar = DocumentGrammar {};
    if !grammar.has_match(&nodes) {
        return ParseError::new("Document is empty").into();
    }
    let document = Document::parse(nodes);

    document
}