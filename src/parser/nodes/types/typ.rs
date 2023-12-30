
use crate::parser::{tree::{TreeNode, TreeNodeLike, ParseError}, grammars::types::simple_type::SimpleTypeGrammar, grammar::GrammarLike};
use super::simple_type::SimpleType;

trait_enum! {
    #[derive(Debug, Clone)]
    pub enum Type: TypeLike {
        SimpleType
    }
}

impl Type {
    pub fn parse(nodes: Vec<TreeNode>) -> TreeNode {
        let simple_type_grammar = SimpleTypeGrammar {};
        let nodes = simple_type_grammar.process_all(nodes);

        if nodes.len() > 1 {
            return ParseError::from_nodes(&nodes, format!("Could not combine types. Multiple types detected where only one is expected.")).into();
        }
        if nodes.len() == 0 {
            return ParseError::from_nodes(&nodes, format!("Could not parse type. Expected type, found nothing.")).into();
        }
        let node = nodes[0].clone();
        
        node
    }
}

pub trait TypeLike {
    fn to_node_like(&self) -> Box<&dyn TreeNodeLike>;
}

impl TreeNodeLike for Type {
    fn get_start_index(&self) -> usize {
        self.to_node_like().get_start_index()
    }
    fn get_end_index(&self) -> usize {
        self.to_node_like().get_end_index()
    }
}