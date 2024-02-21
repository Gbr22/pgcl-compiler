use crate::lexer::types::address_spaces::is_address_space;
use crate::parser::brackets::is_closing_bracket_angle;
use crate::parser::brackets::is_opening_bracket_angle;
use crate::parser::nodes::tagged_string::TaggedString;
use crate::parser::nodes::var_declaration::AstVarDeclaration;
use crate::parser::parse::Parser;
use crate::parser::parsers::types::typ::TypeParser;
use crate::pop_back_node;
use crate::pop_front_node;

use crate::{
    common::range::Range,
    lexer::types::{
        keywords::{is_keyword, VAR},
        token_type::TokenType,
    },
    parser::{
        tree::{ParseError, TreeNode, TreeNodeLike},
        tree_nodes::TreeNodes,
    },
};
pub struct VarDeclarationParser {}

impl Parser for VarDeclarationParser {
    fn parse(mut nodes: TreeNodes) -> TreeNode {
        let range = nodes.range;

        pop_back_node!(
            nodes,
            "Variable declaration must end with semicolon.",
            Some(TreeNode::Token(semi_colon)),
            semi_colon.typ == TokenType::Semicolon
        );

        pop_front_node!(
            nodes,
            "Variable declaration must start with keyword 'var'.",
            Some(TreeNode::Token(var_keyword)),
            var_keyword.typ == TokenType::Identifier && var_keyword.string == VAR
        );

        pop_front_node!(
            nodes,
            "Unexpected end of variable declaration.",
            Some(lt_or_name),
            true
        );

        let has_address_space = is_opening_bracket_angle(&lt_or_name);

        let name_error = "Variable name must be an identifier.";

        let (name, address_space) = if has_address_space {
            pop_front_node!(
                nodes,
                "Expected address space after '<'.",
                Some(TreeNode::Token(token)),
                is_address_space(&token.string)
            );

            pop_front_node!(
                nodes,
                "Expected '>' after address space.",
                Some(node),
                is_closing_bracket_angle(&node)
            );

            pop_front_node!(
                nodes,
                name_error,
                Some(TreeNode::Token(name)),
                name.typ == TokenType::Identifier && !is_keyword(&name.string)
            );

            let name: TaggedString = name.into();

            (name, Some(token.into()))
        } else {
            let TreeNode::Token(name) = lt_or_name else {
                return ParseError::at(lt_or_name.get_range(), name_error).into();
            };
            if !(name.typ == TokenType::Identifier && !is_keyword(&name.string)) {
                return ParseError::at(name.get_range(), name_error).into();
            }

            let name: TaggedString = name.into();

            (name, None)
        };

        let name = name.value;

        pop_front_node!(
            nodes,
            "Colon expected after variable name.",
            Some(TreeNode::Token(colon)),
            colon.typ == TokenType::Colon
        );

        let type_nodes = nodes;
        if type_nodes.len() == 0 {
            return ParseError::at(
                Range::between(colon.get_range(), semi_colon.get_range()),
                "Variable type must not be empty.".to_string(),
            )
            .into();
        }

        let typ = TypeParser::parse(type_nodes);

        TreeNode::VarDeclaration(AstVarDeclaration {
            name,
            range,
            typ: Box::new(typ),
            address_space,
        })
    }
}
