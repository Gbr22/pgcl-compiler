use crate::{tokens::tokenize::{tokenize, Token}, parser::{parse::parse, document::Document, tree::{TreeNode}}, position::get_position};

pub struct AnalizeResult {
    pub tokens: Vec<Token>,
    pub root: TreeNode,
    pub errors: Vec<crate::error::Error>
}

pub fn analize(input: &str) -> AnalizeResult {
    let tokenize_result = tokenize(&input);
    let tokens = tokenize_result.tokens;
    let token_errors = tokenize_result.failed_tokens
        .iter()
        .map(|token|{
            let text = format!("Invalid token {:?}",token);
            crate::error::Error {
                text: text,
                start_pos: get_position(&input, token.start_index),
                end_pos: get_position(&input, token.end_index),
            }
        })
        .collect();
    let root = parse(&tokens);
    let errors = token_errors;

    AnalizeResult {
        root,
        tokens,
        errors
    }
}