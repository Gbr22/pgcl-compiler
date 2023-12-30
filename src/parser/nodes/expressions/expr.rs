use crate::parser::grammar::{GrammarLike, process_grammars};
use crate::parser::grammars::expressions::function_call::FunctionCallGrammar;
use crate::parser::grammars::expressions::value_access::ValueAccessGrammar;
use crate::parser::tree::{TreeNodeLike, TreeNode, ParseError, get_start_index, get_end_index};
use super::value_access::ValueAccess;
use super::function_call::FunctionCall;

trait_enum!{
    #[derive(Debug, Clone)]
    pub enum Expression: ExpressionLike {
        ValueAccess,
        FunctionCall
    }
}

pub trait ExpressionLike {
    fn to_node_like(&self) -> Box<&dyn TreeNodeLike>;
}

impl TreeNodeLike for Expression {
    fn get_start_index(&self) -> usize {
        self.to_node_like().get_start_index()
    }
    fn get_end_index(&self) -> usize {
        self.to_node_like().get_end_index()
    }
    fn get_errors(&self) -> Vec<ParseError> {
        self.to_node_like().get_errors()
    }
}

impl Expression {
    pub fn parse(nodes: Vec<TreeNode>) -> TreeNode {
        let start_index = get_start_index(&nodes).unwrap_or_default();
        let end_index = get_end_index(&nodes).unwrap_or_default();
        
        let nodes = process_grammars(vec![
            FunctionCallGrammar {}.into(),
            ValueAccessGrammar {}.into()
        ], nodes);

        if nodes.len() == 0 {
            return ParseError::at(start_index,end_index,format!("Expected expression")).into();    
        }

        if nodes.len() > 1 {
            return ParseError::at(start_index,end_index,format!("Multiple expressions detected. Expected one.")).into();    
        }

        return nodes[0].clone();
    }
}