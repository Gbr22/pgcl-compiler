
use crate::lexer::types::keywords::is_keyword;
use crate::lexer::types::token_type::TokenType;
use crate::parser::tree::{TreeNode, TreeNodeLike, ParseError, get_start_index, get_end_index};
use super::expressions::expr::Expression;

#[derive(Debug, Clone)]
pub struct FunctionCallArg {
    start_index: usize,
    end_index: usize,
    expr: Box<TreeNode>
}

impl FunctionCallArg {
    pub fn parse(nodes: Vec<TreeNode>) -> TreeNode {
        let start_index = get_start_index(&nodes).unwrap_or_default();
        let end_index = get_end_index(&nodes).unwrap_or_default();
        let mut nodes = nodes;

        let has_comma = match nodes.last() {
            None => false,
            Some(node)=>node.is_token_type(TokenType::Comma)
        };

        if has_comma {
            nodes.pop();
        }

        let expr = Expression::parse(nodes);

        let arg = FunctionCallArg {
            start_index,
            end_index,
            expr: Box::new(expr)
        };

        TreeNode::FunctionCallArg(arg)
    }
}


impl TreeNodeLike for FunctionCallArg {
    fn get_start_index(&self) -> usize {
        self.start_index
    }
    fn get_end_index(&self) -> usize {
        self.end_index
    }
    fn get_errors(&self) -> Vec<ParseError> {
        self.expr.get_errors()
    }
}