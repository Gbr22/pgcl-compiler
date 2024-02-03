use crate::parser::grammars::types::compound::CompoundTypeGrammar;
use crate::parser::grammars::types::simple::SimpleTypeGrammar;

use crate::parser::{parse::Parser, tree::TreeNode, tree_nodes::TreeNodes};
use crate::{process_grammars, try_nodes_into_one_with_message};

pub struct TypeParser {}

impl Parser for TypeParser {
    fn parse(nodes: TreeNodes) -> TreeNode {
        let nodes = process_grammars! { nodes [
            CompoundTypeGrammar,
            SimpleTypeGrammar
        ] };

        try_nodes_into_one_with_message!(
            let node from nodes;
            =0: "Expected type, found nothing.";
            >1: "Could not combine types. Multiple types found where only one is expected.";
        );

        node
    }
}
