use crate::{
    lexer::types::{
        keywords::{is_keyword, FN},
        token_type::TokenType,
    },
    parser::{
        grammars::function_declaration::{find_args_end, find_body_end, find_body_start},
        nodes::function_declaration::FunctionDeclaration,
        parse::Parser,
        parsers::{block::BlockParser, function_args::FunctionArgsParser, types::typ::TypeParser},
        tree::{ParseError, TreeNode},
        tree_nodes::TreeNodes,
    },
    pop_front_node,
};

pub struct FunctionDeclarationParser {}

impl Parser for FunctionDeclarationParser {
    fn parse(mut nodes: TreeNodes) -> TreeNode {
        let range = nodes.range;

        pop_front_node!(
            nodes,
            "Missing fn keyword",
            Some(fn_keyword),
            fn_keyword.is_keyword(FN)
        );

        pop_front_node!(
            nodes,
            "Missing function name.",
            Some(TreeNode::Token(name_node)),
            name_node.typ == TokenType::Identifier && !is_keyword(&name_node.string)
        );

        let name = name_node.string;

        pop_front_node!(
            nodes,
            "Expected opening round bracket after function name.",
            Some(args_open),
            args_open.is_token_type(TokenType::OpeningBracketRound)
        );

        // since the opening bracket has been removed the starting index is shifted by -1
        let Some(args_end_index) = find_args_end(-1, nodes.iter()) else {
            return ParseError::at(
                range,
                "Mismatched brackets. Round bracket `()` not closed.".to_string(),
            )
            .into();
        };
        let argument_nodes = nodes.slice(0, args_end_index);
        nodes.pop_front(); // remove closing bracket `)`

        let args = FunctionArgsParser::parse(argument_nodes);

        pop_front_node!(
            nodes,
            "Expected `->` after function parameters.",
            Some(arrow),
            arrow.is_token_type(TokenType::ArrowRight)
        );

        let Some(body_start_index) = find_body_start(0, nodes.iter()) else {
            return ParseError::at(range, "Could not find start of function body.".to_string())
                .into();
        };
        let Some(body_end_index) = find_body_end(body_start_index, nodes.iter()) else {
            return ParseError::at(
                range,
                "Mismatched brackets. Curly bracket `{}` not closed.".to_string(),
            )
            .into();
        };
        let after_body = nodes.slice(body_end_index, usize::MAX);
        // there should only be a single `}` in the vector
        if after_body.len() > 1 {
            return ParseError::at(range, "Unexpected items after function body.".to_string())
                .into();
        }
        let mut body_nodes = nodes.slice(body_start_index, body_end_index);
        body_nodes.pop_front(); // skip opening curly `{`

        let body_block = BlockParser::parse(body_nodes);

        let type_nodes = nodes;
        let typ = TypeParser::parse(type_nodes);

        TreeNode::FunctionDeclaration(FunctionDeclaration {
            name,
            return_type: Box::new(typ),
            body: Box::new(body_block),
            range,
            args: Box::new(args),
        })
    }
}
