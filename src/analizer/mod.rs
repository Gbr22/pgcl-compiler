use crate::{
    common::range::Range,
    import_resolver::ImportResolver,
    lexer::{token::Token, tokenize::tokenize},
    parser::{
        parse::parse,
        program_tree::program_tree::{create_program_tree, ProgramTree, PtError},
        tree::{ParseError, TreeNode, TreeNodeLike},
    },
    position::{get_position, Position},
};

#[derive(Debug, Clone)]
pub struct AnalizeResult {
    pub tokens: Vec<Token>,
    pub ast: TreeNode,
    pub errors: Vec<crate::error::Error>,
    pub pt: Result<ProgramTree, PtError>,
}

pub fn create_analizer_error(message: String) -> AnalizeResult {
    let parse_err = ParseError::at(Range::new(0, 0), message.clone());
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
    };
}

pub fn analize(import_resolver: &ImportResolver, main_uri: String) -> AnalizeResult {
    let Some(input) = import_resolver.resolve(&main_uri) else {
        return create_analizer_error(format!("Could not resolve file \"{}\"", main_uri));
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

    let pt = create_program_tree(cloned_ast, main_uri);

    AnalizeResult {
        ast,
        tokens,
        errors,
        pt,
    }
}
