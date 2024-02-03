use crate::lexer::definitions::{
    block_comment::BlockCommentDef, exact_match::ExactMatchDef, identifier::IdentifierDef,
    invalid_char::InvalidCharDef, line_comment::LineCommentDef, match_any::MatchAnyDef,
    number::NumberDef, token_def::TokenDef,
};

use super::token_type::TokenType;

pub fn get_definitions() -> &'static [TokenType] {
    TokenType::all_variants()
}

pub fn get_definition(typ: &TokenType) -> Box<dyn TokenDef> {
    use TokenType as T;
    match typ {
        // special
        T::StartOfInput => ExactMatchDef {
            string: "".to_owned(),
        }
        .into(),
        T::InvalidChar => InvalidCharDef {}.into(),

        // whitespace
        T::Whitespace => MatchAnyDef {
            chars: " \t\r".chars().collect(),
        }
        .into(),
        T::Newline => ExactMatchDef {
            string: "\n".to_owned(),
        }
        .into(),

        // number operators
        T::ForwardSlash => ExactMatchDef { string: "/".into() }.into(),
        T::DoubleForwardSlash => ExactMatchDef {
            string: "//".into(),
        }
        .into(),
        T::Star => ExactMatchDef { string: "*".into() }.into(),
        T::Dash => ExactMatchDef { string: "-".into() }.into(),
        T::Plus => ExactMatchDef { string: "+".into() }.into(),

        // logical operators
        T::Bang => ExactMatchDef { string: "!".into() }.into(),

        // comparison operators
        T::DoubleEquals => ExactMatchDef {
            string: "==".into(),
        }
        .into(),
        T::BangEquals => ExactMatchDef {
            string: "!=".into(),
        }
        .into(),
        T::LtEquals => ExactMatchDef {
            string: "<=".into(),
        }
        .into(),
        T::GtEquals => ExactMatchDef {
            string: ">=".into(),
        }
        .into(),
        T::Lt => ExactMatchDef { string: "<".into() }.into(),
        T::Gt => ExactMatchDef { string: ">".into() }.into(),

        // brackets
        T::OpeningBracketRound => ExactMatchDef { string: "(".into() }.into(),
        T::ClosingBracketRound => ExactMatchDef { string: ")".into() }.into(),
        T::OpeningBracketSquare => ExactMatchDef { string: "[".into() }.into(),
        T::ClosingBracketSquare => ExactMatchDef { string: "]".into() }.into(),
        T::OpeningBracketCurly => ExactMatchDef { string: "{".into() }.into(),
        T::ClosingBracketCurly => ExactMatchDef { string: "}".into() }.into(),

        // punctuation
        T::Semicolon => ExactMatchDef { string: ";".into() }.into(),
        T::Colon => ExactMatchDef { string: ":".into() }.into(),
        T::Comma => ExactMatchDef { string: ",".into() }.into(),
        T::Dot => ExactMatchDef { string: ".".into() }.into(),

        // other
        T::Identifier => IdentifierDef {}.into(),

        // values
        T::Number => NumberDef {}.into(),

        // symbols
        T::ArrowRight => ExactMatchDef {
            string: "->".into(),
        }
        .into(),
        T::Equals => ExactMatchDef { string: "=".into() }.into(),

        // comments
        T::LineComment => LineCommentDef {}.into(),
        T::BlockComment => BlockCommentDef {}.into(),
    }
}
