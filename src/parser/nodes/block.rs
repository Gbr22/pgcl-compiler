use crate::{parser::{tree::{TreeNode, TreeNodeLike, ParseError, get_start_index, get_end_index, get_range}, grammars::statements::{simple::SimpleStatementGrammar, ret::ReturnStatementGrammar}, tree_nodes::TreeNodes}, process_grammars, common::range::Range};

#[derive(Debug, Clone)]
pub struct Block {
    range: Range,
    children: Vec<TreeNode>
}

impl Block {
    pub fn parse(nodes: TreeNodes) -> TreeNode {
        let range = nodes.range;

        let nodes = process_grammars! { nodes [
            ReturnStatementGrammar,
            SimpleStatementGrammar
        ] };
        
        let block = Block {
            children: nodes.into_vec(),
            range,
        };

        TreeNode::Block(block)
    }
}

impl TreeNodeLike for Block {
    fn get_range(&self) -> Range {
        self.range
    }
    fn get_errors(&self) -> Vec<ParseError> {
        let mut errors: Vec<ParseError> = vec![];
        for child in &self.children {
            errors.extend(child.get_errors());
        }

        errors
    }
}