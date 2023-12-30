use std::collections::VecDeque;

use crate::{parser::{tree::{TreeNode, TreeNodeLike, ParseError, get_start_index, get_end_index}, grammar::GrammarLike, grammars::{uniform_declaration::UniformDeclarationGrammar, function_declaration::{FunctionDeclarationGrammar}}}, lexer::types::keywords::RETURN};

use super::statement::{StatementLike, Statement};

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    start_index: usize,
    end_index: usize,
}

impl ReturnStatement {
    pub fn parse(nodes: Vec<TreeNode>) -> TreeNode {
        let start_index = get_start_index(&nodes)
            .unwrap_or_default();
        let end_index = get_end_index(&nodes)
            .unwrap_or_default();

        let mut queue: VecDeque<TreeNode> = nodes.into();
        let keyword_error = ParseError::at(start_index, end_index, format!("Expected return keyword."));
        let return_keyword = queue.pop_front();
        let Some(return_keyword) = return_keyword else {
            return keyword_error.into();
        };
        if !return_keyword.is_keyword(RETURN) {
            return return_keyword.into();
        }

        let nodes: Vec<TreeNode> = queue.into();
        // TODO parse expression

        let statement = ReturnStatement {
            start_index,
            end_index
        };

        TreeNode::Statement(Statement::ReturnStatement(statement))
    }
}

impl TreeNodeLike for ReturnStatement {
    fn get_start_index(&self) -> usize {
        self.start_index
    }
    fn get_end_index(&self) -> usize {
        self.end_index
    }
}

impl StatementLike for ReturnStatement {
    fn to_node_like(&self) -> Box<&dyn TreeNodeLike> {
        Box::new(self)
    }
}