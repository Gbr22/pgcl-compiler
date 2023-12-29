use crate::{lexer::types::ignored::is_ignored_token_type, parser::{tree::{TreeNode, TreeNodeLike, ParseError, get_start_index, get_end_index}, grammar::GrammarLike, grammars::{uniform_declaration::UniformDeclarationGrammar, function_declaration::{FunctionDeclarationGrammar, find_args_start, find_args_end}}}};

#[derive(Debug, Clone)]
pub struct Document {
    start_index: usize,
    end_index: usize,
    children: Vec<TreeNode>
}

fn remove_whitespace(nodes: Vec<TreeNode>) -> Vec<TreeNode> {
    let nodes: Vec<TreeNode> = nodes.into_iter().filter(|node|{
        let TreeNode::Token(token) = node else {
            return true;
        };

        if is_ignored_token_type(&token.typ) {
            return false;
        }

        true
    }).collect();

    nodes
}

impl Document {
    pub fn parse(nodes: Vec<TreeNode>) -> TreeNode {
        let start_index = get_start_index(&nodes)
            .unwrap_or_default();
        let end_index = get_end_index(&nodes)
            .unwrap_or_default();

        let uniform_grammar = UniformDeclarationGrammar {};
        let function_grammar = FunctionDeclarationGrammar {};
        let nodes = remove_whitespace(nodes);
        let nodes = uniform_grammar.process_all(nodes);
        let nodes = function_grammar.process_all(nodes);
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