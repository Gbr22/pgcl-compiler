use crate::{
    common::range::Range,
    parser::{
        grammars::statements::{ret::ReturnStatementGrammar, simple::SimpleStatementGrammar},
        tree::{get_end_index, get_range, get_start_index, ParseError, TreeNode, TreeNodeLike},
        tree_nodes::TreeNodes,
    },
    process_grammars,
};

#[derive(Debug, Clone)]
pub struct Block {
    range: Range,
    children: Vec<TreeNode>,
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
