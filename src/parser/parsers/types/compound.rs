use crate::lexer::types::keywords::is_keyword;
use crate::lexer::types::token_type::TokenType;
use crate::parser::nodes::types::compound::CompoundType;
use crate::parser::nodes::types::typ::Type;
use crate::parser::parsers::types::type_args::TypeArgsParser;
use crate::parser::{parse::Parser, tree::TreeNode, tree_nodes::TreeNodes};
use crate::{pop_back_node, pop_front_node};

pub struct CompoundTypeParser {}

impl Parser for CompoundTypeParser {
    fn parse(mut nodes: TreeNodes) -> TreeNode {
        let range = nodes.range;

        pop_front_node!(
            nodes,
            "Expected identifier.",
            Some(TreeNode::Token(token)),
            token.typ == TokenType::Identifier && !is_keyword(&token.string)
        );

        let name = token.string;

        pop_front_node!(
            nodes,
            "Expected `<`",
            Some(TreeNode::Token(token)),
            token.typ == TokenType::Lt
        );

        pop_back_node!(
            nodes,
            "Expected `>`",
            Some(TreeNode::Token(token)),
            token.typ == TokenType::Gt
        );

        let args = nodes;
        let args = TypeArgsParser::parse(args);
        let args = Box::new(args);

        let typ = CompoundType { name, range, args };

        TreeNode::Type(Type::CompoundType(typ))
    }
}
