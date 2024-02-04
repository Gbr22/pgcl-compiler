use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    common::range::Range,
    parser::{
        nodes::document::PtDocument,
        tree::{ParseError, TreeNode},
    },
};
use rayon::{iter::Either, prelude::*, vec::IntoIter};

use super::{
    scope::{Scope, ScopeId},
    value_declaration::ValueDeclarationReferable,
};

#[derive(Debug, Clone)]
pub struct PtError {
    pub range: Option<Range>,
    pub message: String,
}

impl From<ParseError> for PtError {
    fn from(value: ParseError) -> Self {
        PtError {
            range: Some(value.range),
            message: value.text,
        }
    }
}

pub trait TryIntoPt<Final> {
    fn try_into_pt(
        self,
        root_context: Arc<Mutex<RootContext>>,
        context: &CurrentContext,
    ) -> Result<Final, PtError>;
}

pub fn try_map_into_pt<Source, Destination>(
    vec: Vec<Source>,
    root_context: Arc<Mutex<RootContext>>,
    context: &CurrentContext,
) -> Result<Vec<Destination>, PtError>
where
    Destination: Sized + Sync + Send,
    Source: TryIntoPt<Destination> + Sized + Sync + Send,
{
    let iter: IntoIter<Source> = vec.into_par_iter();
    let vecs: (Vec<Destination>, Vec<PtError>) = iter.partition_map(|fun: Source| {
        let result: Result<Destination, PtError> = fun.try_into_pt(root_context.clone(), &context);

        match result {
            Ok(ok) => Either::Left(ok),
            Err(err) => Either::Right(err),
        }
    });

    if let Some(err) = vecs.1.first() {
        return Err(err.clone());
    };

    Ok(vecs.0)
}

#[derive(Debug, Clone)]
pub struct RootContext {
    pub scopes: HashMap<ScopeId, Scope>,
}

pub struct CurrentContext {
    pub uri: String,
    pub accessible_scopes: Vec<ScopeId>,
}

pub fn create_global_scope() -> Scope {
    Scope {
        types: vec![],
        values: vec![],
        functions: vec![],
    }
}

#[derive(Debug)]
pub struct ProgramTree {
    pub context: Arc<Mutex<RootContext>>,
    pub main: PtDocument,
}

pub fn create_program_tree(
    main_document: TreeNode,
    main_uri: String,
) -> Result<ProgramTree, PtError> {
    let TreeNode::Document(doc) = main_document else {
        return Err(PtError {
            range: Some(main_document.get_range()),
            message: format!("Expected document."),
        });
    };

    let mut scopes = HashMap::new();
    scopes.insert(ScopeId::Global, create_global_scope());

    let root_context = Arc::new(Mutex::new(RootContext { scopes }));

    let current_context = CurrentContext {
        uri: main_uri,
        accessible_scopes: vec![ScopeId::Global],
    };

    let main: PtDocument = doc.try_into_pt(root_context.clone(), &current_context)?;

    Ok(ProgramTree {
        context: root_context,
        main,
    })
}
