use crate::parser::{tree::{TreeNode, TreeNodeLike, ParseError, get_start_index, get_end_index}, grammar::GrammarLike, grammars::{uniform_declaration::UniformDeclarationGrammar, function_declaration::{FunctionDeclarationGrammar}}};

// Semicolon delimited statement
#[derive(Debug, Clone)]
pub struct SimpleStatement {
    start_index: usize,
    end_index: usize,
}

impl SimpleStatement {
    pub fn parse(nodes: Vec<TreeNode>) -> TreeNode {
        let start_index = get_start_index(&nodes)
            .unwrap_or_default();
        let end_index = get_end_index(&nodes)
            .unwrap_or_default();

        // TODO parse expressions

        let statement = SimpleStatement {
            start_index,
            end_index
        };

        TreeNode::SimpleStatement(statement)
    }
}

impl TreeNodeLike for SimpleStatement {
    fn get_start_index(&self) -> usize {
        self.start_index
    }
    fn get_end_index(&self) -> usize {
        self.end_index
    }
}