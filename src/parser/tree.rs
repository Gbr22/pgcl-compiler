use crate::common::range::Range;
use crate::lexer::token::Token;
use crate::lexer::types::token_type::TokenType;
use crate::parser::nodes::block::Block;
use crate::parser::nodes::document::AstDocument as Document;
use crate::parser::nodes::expressions::expr::Expression;
use crate::parser::nodes::function_arg::FunctionArg;
use crate::parser::nodes::function_args::FunctionArgs;
use crate::parser::nodes::function_call_arg::FunctionCallArg;
use crate::parser::nodes::function_call_args::FunctionCallArgs;
use crate::parser::nodes::function_declaration::AstFunctionDeclaration as FunctionDeclaration;
use crate::parser::nodes::statements::statement::Statement;
use crate::parser::nodes::types::typ::AstType;
use crate::parser::nodes::types::type_arg::TypeArg;
use crate::parser::nodes::types::type_args::TypeArgs;
use crate::parser::nodes::uniform_declaration::AstUniformDeclaration as UniformDeclaration;

trait_enum! {
    #[derive(Debug, Clone)]
    pub enum TreeNode: TreeNodeLike {
        Token,
        Document,
        UniformDeclaration,
        ParseError,
        AstType,
        FunctionDeclaration,
        Block,
        Statement,
        Expression,
        FunctionCallArgs,
        FunctionCallArg,
        FunctionArg,
        FunctionArgs,
        TypeArg,
        TypeArgs
    }
}

impl TreeNode {
    pub fn is_token_type(&self, typ: TokenType) -> bool {
        let TreeNode::Token(token) = self else {
            return false;
        };

        token.typ == typ
    }
    pub fn is_keyword(&self, str: &str) -> bool {
        let TreeNode::Token(token) = self else {
            return false;
        };

        token.typ == TokenType::Identifier && token.string == str
    }
    pub fn is_error(&self) -> bool {
        if let TreeNode::ParseError(_) = self {
            true
        } else {
            false
        }
    }
    pub fn to_string(&self) -> String {
        let string = format!("{:#?}", self);

        let string = string
            .split('\n')
            .map(|line| {
                let space_count = line.chars().take_while(|char| char == &' ').count();

                let rest: String = line.chars().skip_while(|char| char == &' ').collect();

                let new_space_count = space_count / 4;
                let new_spaces: String = (0..new_space_count).map(|_| ' ').collect();

                let new_string = format!("{}{}", new_spaces, rest);

                new_string
            })
            .collect::<Vec<String>>()
            .join("\n");

        string
    }
}

pub trait TreeNodeLike: Sync {
    fn get_range(&self) -> Range;
    fn get_errors(&self) -> Vec<&ParseError> {
        self.descendants()
            .into_iter()
            .filter_map(|n| {
                if let TreeNode::ParseError(err) = n {
                    return Some(err);
                }
                None
            })
            .collect()
    }
    fn children(&self) -> Vec<&TreeNode>;
    fn descendants(&self) -> Vec<&TreeNode> {
        let children: Vec<&TreeNode> = self.children();
        let mut descendants: Vec<&TreeNode> = vec![];

        for child in children {
            descendants.push(child);
            descendants.extend(child.descendants())
        }

        descendants
    }
    fn iter(&self) -> <Vec<&TreeNode> as IntoIterator>::IntoIter {
        return self.descendants().into_iter();
    }
}

impl TreeNodeLike for Token {
    fn get_range(&self) -> Range {
        self.range
    }
    fn children(&self) -> Vec<&TreeNode> {
        vec![]
    }
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub text: String,
    pub range: Range,
}

impl ParseError {
    pub fn new(text: impl Into<String>) -> ParseError {
        ParseError {
            text: text.into(),
            range: Range::null(),
        }
    }
    pub fn at(range: Range, text: impl Into<String>) -> ParseError {
        ParseError {
            range,
            text: text.into(),
        }
    }
    pub fn from_nodes(nodes: &[TreeNode], text: impl Into<String>) -> ParseError {
        let text = text.into();
        let range = get_range(nodes).unwrap_or(Range::null());

        ParseError { text, range }
    }
}

impl From<ParseError> for TreeNode {
    fn from(val: ParseError) -> Self {
        TreeNode::ParseError(val)
    }
}

impl TreeNodeLike for ParseError {
    fn get_range(&self) -> Range {
        self.range
    }
    fn children(&self) -> Vec<&TreeNode> {
        vec![]
    }
}

pub fn get_range(nodes: &[TreeNode]) -> Option<Range> {
    let start_index = get_start_index(nodes)?;
    let end_index = get_end_index(nodes)?;

    Some(Range::new(start_index, end_index))
}

pub fn get_start_index(nodes: &[TreeNode]) -> Option<usize> {
    nodes.first().map(|f| f.get_range().start_index)
}
pub fn get_end_index(nodes: &[TreeNode]) -> Option<usize> {
    nodes.last().map(|f| f.get_range().end_index)
}
