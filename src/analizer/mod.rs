use crate::{
    lexer::{token::Token, tokenize::tokenize},
    parser::{
        parse::parse,
        tree::{TreeNode, TreeNodeLike},
    },
    position::get_position,
};

pub struct AnalizeResult {
    pub tokens: Vec<Token>,
    pub root: TreeNode,
    pub errors: Vec<crate::error::Error>,
}

pub fn analize(input: &str) -> AnalizeResult {
    let tokenize_result = tokenize(&input);
    let tokens = tokenize_result.tokens;
    let token_errors: Vec<crate::error::Error> = tokenize_result
        .failed_tokens
        .iter()
        .map(|token| {
            let text = token.get_error_message();
            crate::error::Error {
                text: text,
                start_pos: get_position(&input, token.range.start_index),
                end_pos: get_position(&input, token.range.end_index),
            }
        })
        .collect();
    let root = parse(&tokens);
    let parse_errors = root.get_errors();
    let parse_errors = parse_errors.iter().map(|e| crate::error::Error {
        text: e.text.to_owned(),
        start_pos: get_position(&input, e.get_range().start_index),
        end_pos: get_position(&input, e.get_range().end_index),
    });
    let mut errors: Vec<crate::error::Error> = vec![];
    errors.extend(token_errors);
    errors.extend(parse_errors);

    AnalizeResult {
        root,
        tokens,
        errors,
    }
}
