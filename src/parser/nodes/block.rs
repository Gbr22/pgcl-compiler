use crate::{lexer::types::ignored::is_ignored_token_type, parser::{tree::{TreeNode, TreeNodeLike, ParseError, get_start_index, get_end_index}, grammar::GrammarLike, grammars::{uniform_declaration::UniformDeclarationGrammar, function_declaration::{FunctionDeclarationGrammar, find_args_start, find_args_end}, statements::simple_statement::SimpleStatementGrammar}}};

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

        let simple_statement_grammar = SimpleStatementGrammar {};
        let nodes = simple_statement_grammar.process_all(nodes);
        
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