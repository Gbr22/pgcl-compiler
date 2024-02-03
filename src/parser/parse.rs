use crate::{
    common::range::Range,
    lexer::{token::Token, types::ignored::is_ignored_token_type},
};

use super::{
    grammar::GrammarLike,
    grammars::document::DocumentGrammar,
    nodes::document::Document,
    tree::{get_range, ParseError, TreeNode},
    tree_nodes::TreeNodes,
};

fn remove_whitespace(nodes: Vec<TreeNode>) -> Vec<TreeNode> {
    let nodes: Vec<TreeNode> = nodes
        .into_iter()
        .filter(|node| {
            let TreeNode::Token(token) = node else {
            return true;
        };

            if is_ignored_token_type(&token.typ) {
                return false;
            }

            true
        })
        .collect();

    nodes
}

pub fn parse(tokens: &[Token]) -> TreeNode {
    let nodes: Vec<TreeNode> = tokens
        .iter()
        .map(|token| TreeNode::Token(token.clone()))
        .collect();
    let range = get_range(&nodes).unwrap_or(Range::null());
    let nodes = remove_whitespace(nodes);
    let nodes = TreeNodes::new(range, nodes);

    let grammar = DocumentGrammar {};
    if !grammar.has_match(&nodes) {
        return ParseError::new("Document is empty").into();
    }
    let document = Document::parse(nodes);

    document
}

pub trait Parser {
    fn parse(nodes: TreeNodes) -> TreeNode;
}

#[macro_export]
macro_rules! use_parser {
    ($struct_name:ident) => {
        fn construct(&self, nodes: TreeNodes) -> TreeNode {
            let result = $struct_name::parse(nodes);

            result
        }
    };
}
