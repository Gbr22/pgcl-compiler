use crate::tokens::tokenize::{Token, TokenType};

use super::tree::{TreeNodeLike, TreeNode};

#[derive(Debug, Clone)]
pub struct Document {
    start_index: usize,
    end_index: usize,
    children: Vec<TreeNode>
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
        
        // TODO: parse children
        
        let document = Document {
            children: nodes,
            start_index,
            end_index
        };

        TreeNode::Document(document)
    }
   /*  fn parse_uniforms(mut self) -> Self {
        loop {
            let mut uniform_index: Option<usize> = None;
            for (index, item) in self.children.iter().enumerate() {
                let TreeNode::Token(token) = item else {
                    continue
                };
                if token.get_string() != UNIFORM_KEYWORD {
                    continue;
                }
                uniform_index = Some(index);
                break;
            }
            let Some(start_index) = uniform_index else {
                break;
            };

            let mut end_index: Option<usize> = None;
            for (index, item) in self.children.iter().enumerate() {
                let TreeNode::Token(token) = item else {
                    continue;
                };
                if token.typ != TokenType::Semicolon {
                    continue;
                }
                end_index = Some(index);
            }
            let Some(end_index) = end_index else {
                break;
            };
            let range: Vec<TreeNode> = self.children
                .splice(start_index..(end_index+1), vec![])
                .collect();

            /* let element = 
            self.children.insert(start_index, element) */
        }
        
        self
    } */
}

impl TreeNodeLike for Document {
    fn get_start_index(&self) -> usize {
        self.start_index
    }
    fn get_end_index(&self) -> usize {
        self.end_index
    }
}