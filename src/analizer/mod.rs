use wasm_bindgen::convert::OptionIntoWasmAbi;

use crate::{
    common::range::Range, import_resolver::ImportResolver, lexer::{token::Token, tokenize::tokenize}, parser::{
        nodes::document::PtDocument,
        parse::parse,
        program_tree::PtError,
        tree::{ParseError, TreeNode, TreeNodeLike},
    }, position::{get_position, Position}
};

pub struct AnalizeResult {
    pub tokens: Vec<Token>,
    pub ast: TreeNode,
    pub errors: Vec<crate::error::Error>,
    pub pt: Result<PtDocument, PtError>,
}

pub fn analize(import_resolver: &ImportResolver, main_uri: String) -> AnalizeResult {
    let Some(input) = import_resolver.resolve(&main_uri) else {
        let message = format!("Could not resolve file \"{}\"",main_uri);
        let parse_err = ParseError::at(
            Range::new(0, 0),
            message.clone()
        );
        let pt_err: PtError = parse_err.clone().into();
        let crate_err = crate::error::Error {
            text: message,
            start_pos: Position { row: 0, col: 0 },
            end_pos: Position { row: 0, col: 0 },
        };
        return AnalizeResult {
            ast: parse_err.into(),
            tokens: vec![],
            errors: vec![crate_err],
            pt: Err(pt_err),
        }
    };
    let tokenize_result = tokenize(&input);
    let tokens = tokenize_result.tokens;
    let token_errors: Vec<crate::error::Error> = tokenize_result
        .failed_tokens
        .iter()
        .map(|token| {
            let text = token.get_error_message();
            crate::error::Error {
                text,
                start_pos: get_position(&input, token.range.start_index),
                end_pos: get_position(&input, token.range.end_index),
            }
        })
        .collect();
    let ast = parse(&tokens);
    let parse_errors = ast.get_errors();
    let parse_errors = parse_errors.iter().map(|e| crate::error::Error {
        text: e.text.to_owned(),
        start_pos: get_position(&input, e.get_range().start_index),
        end_pos: get_position(&input, e.get_range().end_index),
    });
    let mut errors: Vec<crate::error::Error> = vec![];
    errors.extend(token_errors);
    errors.extend(parse_errors);

    let cloned_ast = ast.clone();

    let pt: Result<PtDocument, PtError> = if let TreeNode::Document(document) = cloned_ast {
        document.try_into()
    } else {
        Err(PtError {
            range: Some(cloned_ast.get_range()),
            message: "Expected document.".to_owned(),
        })
    };

    AnalizeResult {
        ast,
        tokens,
        errors,
        pt,
    }
}
