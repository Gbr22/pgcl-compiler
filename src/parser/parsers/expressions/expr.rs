use crate::parser::grammars::expressions::function_call::FunctionCallGrammar;
use crate::parser::grammars::expressions::value_access::ValueAccessGrammar;
use crate::parser::tree::{ParseError, TreeNode};
use crate::parser::{parse::Parser, tree_nodes::TreeNodes};
use crate::process_grammars;
pub struct ExpressionParser {}

impl Parser for ExpressionParser {
    fn parse(nodes: TreeNodes) -> TreeNode {
        let range = nodes.range;

        let nodes = process_grammars! { nodes [
            FunctionCallGrammar,
            ValueAccessGrammar
        ] };

        if nodes.len() == 0 {
            return ParseError::at(range, format!("Expected expression")).into();
        }

        if nodes.len() > 1 {
            return ParseError::at(
                range,
                format!("Multiple expressions detected. Expected one."),
            )
            .into();
        }

        return nodes.into_first().unwrap();
    }
}
