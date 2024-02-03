use crate::{
    lexer::types::{keywords::is_keyword, token_type::TokenType},
    parser::{
        nodes::{
            expressions::{expr::Expression, function_call::FunctionCall},
            function_call_args::FunctionCallArgs,
        },
        parse::Parser,
        tree::TreeNode,
        tree_nodes::TreeNodes,
    },
    pop_back_node, pop_front_node,
};

pub struct FunctionCallParser {}

impl Parser for FunctionCallParser {
    fn parse(mut nodes: TreeNodes) -> TreeNode {
        let range = nodes.range;

        pop_front_node!(
            nodes,
            "Expected identifier at function call.",
            Some(TreeNode::Token(name)),
            name.typ == TokenType::Identifier && !is_keyword(&name.string)
        );

        let name = name.string;

        pop_front_node!(
            nodes,
            "Expected opening bracket `(` at function call.",
            Some(TreeNode::Token(opening_bracket)),
            opening_bracket.typ == TokenType::OpeningBracketRound
        );

        pop_back_node!(
            nodes,
            "Mismatched brackets. Expected closing round bracket `()` at function call.",
            Some(TreeNode::Token(opening_bracket)),
            opening_bracket.typ == TokenType::ClosingBracketRound
        );

        let arg_nodes: TreeNodes = nodes;
        let args = FunctionCallArgs::parse(arg_nodes);

        let call = FunctionCall {
            name: name,
            range,
            args: Box::new(args),
        };

        TreeNode::Expression(Expression::FunctionCall(call))
    }
}