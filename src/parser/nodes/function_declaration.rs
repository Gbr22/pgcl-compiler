use super::{block::Block, types::typ::Type};
use crate::pop_front_node;
use crate::{
    common::range::Range,
    lexer::types::{
        keywords::{is_keyword, FN, UNIFORM},
        token_type::TokenType,
    },
    parser::{
        grammars::function_declaration::{find_args_end, find_body_end, find_body_start},
        tree::{get_end_index, get_range, get_start_index, ParseError, TreeNode, TreeNodeLike},
        tree_nodes::TreeNodes,
    },
};

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    name: String,
    return_type: Box<TreeNode>,
    body: Box<TreeNode>,
    range: Range,
}

impl FunctionDeclaration {
    pub fn parse(mut nodes: TreeNodes) -> TreeNode {
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

        let Some(args_end_index) = find_args_end(0, nodes.iter()) else {
            return ParseError::at(range,format!("Mismatched brackets. Round bracket `()` not closed.")).into();
        };
        let argument_nodes = nodes.slice(0, args_end_index);
        nodes.pop_front(); // remove closing bracket `)`

        // TODO parse arguments

        pop_front_node!(
            nodes,
            "Expected `->` after function parameters.",
            Some(arrow),
            arrow.is_token_type(TokenType::ArrowRight)
        );

        let Some(body_start_index) = find_body_start(0, nodes.iter()) else {
            return ParseError::at(range, format!("Could not find start of function body.")).into();
        };
        let Some(body_end_index) = find_body_end(body_start_index, nodes.iter()) else {
            return ParseError::at(range, format!("Mismatched brackets. Curly bracket `{{}}` not closed.")).into();
        };
        let after_body = nodes.slice(body_end_index, usize::MAX);
        // there should only be a single `}` in the vector
        if after_body.len() > 1 {
            return ParseError::at(range, format!("Unexpected items after function body.")).into();
        }
        let mut body_nodes = nodes.slice(body_start_index, body_end_index);
        body_nodes.pop_front(); // skip opening curly `{`

        let body_block = Block::parse(body_nodes);

        let type_nodes = nodes;
        let typ = Type::parse(type_nodes);

        TreeNode::FunctionDeclaration(FunctionDeclaration {
            name,
            return_type: Box::new(typ),
            body: Box::new(body_block),
            range,
        })
    }
}

impl TreeNodeLike for FunctionDeclaration {
    fn get_range(&self) -> Range {
        self.range
    }
    fn get_errors(&self) -> Vec<ParseError> {
        let mut errors: Vec<ParseError> = vec![];
        errors.extend(self.return_type.get_errors());
        errors.extend(self.body.get_errors());

        errors
    }
}
