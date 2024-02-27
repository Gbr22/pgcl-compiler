use crate::{
    lexer::types::{keywords::is_keyword, token_type::TokenType},
    parser::{
        nodes::expressions::{expr::Expression, function_call::AstFunctionCall},
        parse::Parser,
        parsers::function_call_args::FunctionCallArgsParser,
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

        let name_range = name.range;
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
        let args = FunctionCallArgsParser::parse(arg_nodes);

        let call = AstFunctionCall {
            name,
            name_range,
            range,
            args: Box::new(args),
        };

        TreeNode::Expression(Expression::FunctionCall(call))
    }
}
