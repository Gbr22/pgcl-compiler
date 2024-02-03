use crate::parser::grammars::function_declaration::FunctionDeclarationGrammar;
use crate::parser::grammars::uniform_declaration::UniformDeclarationGrammar;
use crate::parser::{
    nodes::document::Document, parse::Parser, tree::TreeNode, tree_nodes::TreeNodes,
};
use crate::process_grammars;
pub struct DocumentParser {}

impl Parser for DocumentParser {
    fn parse(nodes: TreeNodes) -> TreeNode {
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
