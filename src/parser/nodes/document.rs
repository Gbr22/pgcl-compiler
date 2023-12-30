use crate::parser::{tree::{TreeNode, TreeNodeLike, ParseError, get_start_index, get_end_index}, grammar::{GrammarLike, process_grammars}, grammars::{uniform_declaration::UniformDeclarationGrammar, function_declaration::{FunctionDeclarationGrammar, find_args_start, find_args_end}}};

#[derive(Debug, Clone)]
pub struct Document {
    start_index: usize,
    end_index: usize,
    children: Vec<TreeNode>
}

impl Document {
    pub fn parse(nodes: Vec<TreeNode>) -> TreeNode {
        let start_index = get_start_index(&nodes)
            .unwrap_or_default();
        let end_index = get_end_index(&nodes)
            .unwrap_or_default();

        let nodes = process_grammars(vec![
            UniformDeclarationGrammar {}.into(),
            FunctionDeclarationGrammar {}.into(),
        ], nodes);
        
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
        let mut errors: Vec<ParseError> = vec![];
        for child in &self.children {
            errors.extend(child.get_errors());
        }

        errors
    }
}