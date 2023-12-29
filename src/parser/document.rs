
use std::clone;

use crate::lexer::types::token_type::TokenType;

use super::{tree::{TreeNodeLike, TreeNode, ParseError}, uniform::UniformGrammar, grammar::GrammarLike};

#[derive(Debug, Clone)]
pub struct Document {
    start_index: usize,
    end_index: usize,
    children: Vec<TreeNode>
}

fn remove_whitespace(nodes: Vec<TreeNode>) -> Vec<TreeNode> {
    let nodes: Vec<TreeNode> = nodes.into_iter().filter(|node|{
        let TreeNode::Token(token) = node else {
            return true;
        };

        let filtered_types: Vec<TokenType> = vec![
            TokenType::Whitespace,
            TokenType::Newline,
            TokenType::InvalidChar,
            TokenType::StartOfInput
        ];

        if filtered_types.contains(&token.typ) {
            return false;
        }

        true
    }).collect();

    nodes
}

impl Document {
    pub fn parse(nodes: Vec<TreeNode>) -> TreeNode {
        let start_index = nodes
            .first()
            .map(|node|node.get_start_index())
            .unwrap_or_default();
        let end_index = nodes
            .first()
            .map(|node|node.get_end_index())
            .unwrap_or_default();

        let nodes = remove_whitespace(nodes);
        let uniform_grammar = UniformGrammar {};
        let nodes = uniform_grammar.process_all(nodes);
        
        let document = Document {
            children: nodes,
            start_index,
            end_index
        };

        TreeNode::Document(document)
    }
}

impl TreeNodeLike for Document {
    fn get_start_index(&self) -> usize {
        self.start_index
    }
    fn get_end_index(&self) -> usize {
        self.end_index
    }
    fn get_errors(&self) -> Vec<ParseError> {
        let errors: Vec<ParseError> = self.children.iter().filter_map(|node|{
            if let TreeNode::ParseError(error) = node {
                return Some(error.to_owned());
            }

            None
        }).collect();

        errors
    }
}