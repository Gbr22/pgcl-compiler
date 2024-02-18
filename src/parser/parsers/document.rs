use crate::parser::grammars::function_declaration::FunctionDeclarationGrammar;
use crate::parser::grammars::var_declaration::VarDeclarationGrammar;
use crate::parser::{
    nodes::document::AstDocument, parse::Parser, tree::TreeNode, tree_nodes::TreeNodes,
};
use crate::process_grammars;
pub struct DocumentParser {}

impl Parser for DocumentParser {
    fn parse(nodes: TreeNodes) -> TreeNode {
        let range = nodes.range;

        let nodes = process_grammars! { nodes [
            VarDeclarationGrammar,
            FunctionDeclarationGrammar
        ] };

        let document = AstDocument {
            range,
            children: nodes.into_vec(),
        };

        TreeNode::Document(document)
    }
}
