use crate::{common::range::Range, lexer::token::Token};

#[derive(Debug, Clone)]
pub struct TaggedString {
    pub range: Range,
    pub value: String,
}

impl TaggedString {
    pub fn at(range: Range, value: impl Into<String>) -> TaggedString {
        TaggedString {
            range,
            value: value.into(),
        }
    }
}

impl From<Token> for TaggedString {
    fn from(token: Token) -> Self {
        TaggedString {
            range: token.range,
            value: token.string,
        }
    }
}
