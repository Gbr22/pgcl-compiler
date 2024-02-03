use crate::{
    common::range::Range,
    parser::{
        grammar::GrammarLike,
        grammars::{
            function_declaration::{find_args_end, find_args_start, FunctionDeclarationGrammar},
            uniform_declaration::UniformDeclarationGrammar,
        },
        tree::{get_end_index, get_range, get_start_index, ParseError, TreeNode, TreeNodeLike},
        tree_nodes::TreeNodes,
    },
    process_grammars,
};

#[derive(Debug, Clone)]
pub struct Document {
    pub range: Range,
    pub children: Vec<TreeNode>,
}

impl Document {}

impl TreeNodeLike for Document {
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
