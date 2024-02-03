use super::function_call::FunctionCall;
use super::value_access::ValueAccess;
use crate::common::range::Range;
use crate::parser::grammars::expressions::function_call::FunctionCallGrammar;
use crate::parser::grammars::expressions::value_access::ValueAccessGrammar;
use crate::parser::tree::{
    get_end_index, get_range, get_start_index, ParseError, TreeNode, TreeNodeLike,
};
use crate::parser::tree_nodes::TreeNodes;
use crate::process_grammars;

trait_enum! {
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
    fn get_range(&self) -> crate::common::range::Range {
        self.to_node_like().get_range()
    }
    fn get_errors(&self) -> Vec<ParseError> {
        self.to_node_like().get_errors()
    }
}

impl Expression {}
