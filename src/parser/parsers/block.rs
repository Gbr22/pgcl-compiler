use crate::parser::grammars::statements::ret::ReturnStatementGrammar;
use crate::parser::grammars::statements::simple::SimpleStatementGrammar;
use crate::parser::grammars::var_declaration::VarDeclarationGrammar;
use crate::parser::{nodes::block::Block, parse::Parser, tree::TreeNode, tree_nodes::TreeNodes};
use crate::process_grammars;

pub struct BlockParser {}

impl Parser for BlockParser {
    fn parse(nodes: TreeNodes) -> TreeNode {
        let range = nodes.range;

        let nodes = process_grammars! { nodes [
            VarDeclarationGrammar,
            ReturnStatementGrammar,
            SimpleStatementGrammar
        ] };

        let block = Block {
            children: nodes.into_vec(),
            range,
        };

        TreeNode::Block(block)
    }
}
