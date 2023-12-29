use enum_all_variants::AllVariants;
use serde_derive::Serialize;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(AllVariants, Debug, Serialize, Clone, Copy, PartialEq)]
pub enum TokenType {
    StartOfInput,
    Identifier,
    Whitespace,
    Newline,
    Number,
    InvalidChar,
    Semicolon,
    Colon,
    Comma,
    Dot,
    ForwardSlash,
    DoubleForwardSlash,
    Star,
    Dash,
    Plus,
    Bang,
    Equals,
    DoubleEquals,
    BangEquals,
    LtEquals,
    GtEquals,
    Lt,
    Gt,
    ArrowRight,
    OpeningBracketRound,
    ClosingBracketRound,
    OpeningBracketSquare,
    ClosingBracketSquare,
    OpeningBracketCurly,
    ClosingBracketCurly,
    LineComment,
    BlockComment
}