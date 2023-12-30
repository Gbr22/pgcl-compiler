use crate::{parser::{tree::{TreeNode, TreeNodeLike, ParseError, get_start_index, get_end_index}, grammars::statements::{simple::SimpleStatementGrammar, ret::ReturnStatementGrammar}}, process_grammars};

#[derive(Debug, Clone)]
pub struct Block {
    start_index: usize,
    end_index: usize,
    children: Vec<TreeNode>
}

impl Block {
    pub fn parse(nodes: Vec<TreeNode>) -> TreeNode {
        let start_index = get_start_index(&nodes)
            .unwrap_or_default();
        let end_index = get_end_index(&nodes)
            .unwrap_or_default();

        let nodes = process_grammars! { nodes [
            ReturnStatementGrammar,
            SimpleStatementGrammar
        ] };
        
        let block = Block {
            children: nodes,
            start_index,
            end_index
        };

        TreeNode::Block(block)
    }
}

impl TreeNodeLike for Block {
    fn get_start_index(&self) -> usize {
        self.start_index
    }
    fn get_end_index(&self) -> usize {
        self.end_index
    }
    fn get_errors(&self) -> Vec<ParseError> {
        let mut errors: Vec<ParseError> = vec![];
        for child in &self.children {
            errors.extend(child.get_errors());
        }

        errors
    }
}