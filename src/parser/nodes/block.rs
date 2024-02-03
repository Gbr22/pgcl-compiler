use crate::{
    common::range::Range,
    parser::{
        grammars::statements::{ret::ReturnStatementGrammar, simple::SimpleStatementGrammar},
        tree::{ParseError, TreeNode, TreeNodeLike},
        tree_nodes::TreeNodes,
    },
    process_grammars,
};

#[derive(Debug, Clone)]
pub struct Block {
    pub range: Range,
    pub children: Vec<TreeNode>,
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
