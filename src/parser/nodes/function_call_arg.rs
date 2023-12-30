
use crate::common::range::Range;
use crate::lexer::types::keywords::is_keyword;
use crate::lexer::types::token_type::TokenType;
use crate::parser::tree::{TreeNode, TreeNodeLike, ParseError, get_start_index, get_end_index, get_range};
use crate::parser::tree_nodes::TreeNodes;
use super::expressions::expr::Expression;

#[derive(Debug, Clone)]
pub struct FunctionCallArg {
    expr: Box<TreeNode>,
    range: Range
}

impl FunctionCallArg {
    pub fn parse(mut nodes: TreeNodes) -> TreeNode {
        let range = nodes.range;

        let has_comma = match nodes.last() {
            None => false,
            Some(node)=>node.is_token_type(TokenType::Comma)
        };

        if has_comma {
            nodes.pop_back();
        }

        let expr = Expression::parse(nodes);

        let arg = FunctionCallArg {
            expr: Box::new(expr),
            range
        };

        TreeNode::FunctionCallArg(arg)
    }
}


impl TreeNodeLike for FunctionCallArg {
    fn get_range(&self) -> Range {
        self.range
    }
    fn get_errors(&self) -> Vec<ParseError> {
        self.expr.get_errors()
    }
}