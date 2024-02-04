use crate::parser::nodes::uniform_declaration::AstUniformDeclaration;
use crate::parser::parse::Parser;
use crate::parser::parsers::types::typ::TypeParser;
use crate::pop_back_node;
use crate::pop_front_node;

use crate::{
    common::range::Range,
    lexer::types::{
        keywords::{is_keyword, UNIFORM},
        token_type::TokenType,
    },
    parser::{
        tree::{ParseError, TreeNode, TreeNodeLike},
        tree_nodes::TreeNodes,
    },
};
pub struct UniformDeclarationParser {}

impl Parser for UniformDeclarationParser {
    fn parse(mut nodes: TreeNodes) -> TreeNode {
        let range = nodes.range;

        pop_back_node!(
            nodes,
            "Uniform declaration must end with semicolon.",
            Some(TreeNode::Token(semi_colon)),
            semi_colon.typ == TokenType::Semicolon
        );

        pop_front_node!(
            nodes,
            "Uniform declaration must start with keyword 'uniform'.",
            Some(TreeNode::Token(uniform_keyword)),
            uniform_keyword.typ == TokenType::Identifier && uniform_keyword.string == UNIFORM
        );

        pop_front_node!(
            nodes,
            "Uniform name must be an identifier.",
            Some(TreeNode::Token(name)),
            name.typ == TokenType::Identifier && !is_keyword(&name.string)
        );

        let name = name.string;

        pop_front_node!(
            nodes,
            "Colon expected after uniform name.",
            Some(TreeNode::Token(colon)),
            colon.typ == TokenType::Colon
        );

        let type_nodes = nodes;
        if type_nodes.len() == 0 {
            return ParseError::at(
                Range::between(colon.get_range(), semi_colon.get_range()),
                "Uniform type must not be empty.".to_string(),
            )
            .into();
        }

        let typ = TypeParser::parse(type_nodes);

        TreeNode::UniformDeclaration(AstUniformDeclaration {
            name,
            range,
            typ: Box::new(typ),
        })
    }
}
