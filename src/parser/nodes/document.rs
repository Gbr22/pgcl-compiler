use crate::{parser::{tree::{TreeNode, TreeNodeLike, ParseError, get_start_index, get_end_index, get_range}, grammar::{GrammarLike}, grammars::{uniform_declaration::UniformDeclarationGrammar, function_declaration::{FunctionDeclarationGrammar, find_args_start, find_args_end}}, tree_nodes::TreeNodes}, process_grammars, common::range::Range};

#[derive(Debug, Clone)]
pub struct Document {
    range: Range,
    children: Vec<TreeNode>
}

impl Document {
    pub fn parse(nodes: TreeNodes) -> TreeNode {
        let range = nodes.range;

        let nodes = process_grammars! { nodes [
            UniformDeclarationGrammar,
            FunctionDeclarationGrammar
        ] };
        
        let document = Document {
            range,
            children: nodes.into_vec(),
        };

        TreeNode::Document(document)
    }
}

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