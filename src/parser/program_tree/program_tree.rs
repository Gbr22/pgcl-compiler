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
    scope::{Scope, ScopeId}, type_declaration::{PrimitiveTypeDeclaration, TypeDeclarationReferable}, value_declaration::ValueDeclarationReferable
};

#[derive(Debug, Clone)]
pub struct PtError {
    pub uri: Option<String>,
    pub range: Option<Range>,
    pub message: String,
}

impl PtError {
    pub fn at(range: Range, message: impl Into<String>) -> PtError {
        PtError {
            range: Some(range),
            message: message.into(),
            uri: None,
        }
    }
    pub fn in_at(uri: impl Into<String>, range: Range, message: impl Into<String>) -> PtError {
        PtError {
            uri: Some(uri.into()),
            range: Some(range),
            message: message.into(),
        }
    }
}

impl From<ParseError> for PtError {
    fn from(value: ParseError) -> Self {
        PtError {
            range: Some(value.range),
            message: value.text,
            uri: None,
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

pub type RootContextRef = Arc<Mutex<RootContext>>;

pub struct CurrentContext {
    pub uri: String,
    pub accessible_scopes: Vec<ScopeId>,
}

pub fn create_global_scope() -> Scope {
    Scope {
        types: vec![
            TypeDeclarationReferable::Primitive(PrimitiveTypeDeclaration {
                name: "f32".to_owned(),
                description: Some("32-bit floating point value".to_owned())
            }),
            TypeDeclarationReferable::Primitive(PrimitiveTypeDeclaration {
                name: "i32".to_owned(),
                description: Some("32-bit signed integer".to_owned())
            }),
            TypeDeclarationReferable::Primitive(PrimitiveTypeDeclaration {
                name: "u32".to_owned(),
                description: Some("32-bit unsigned integer".to_owned())
            }),
            TypeDeclarationReferable::Primitive(PrimitiveTypeDeclaration {
                name: "bool".to_owned(),
                description: Some("boolean (true or false) value".to_owned())
            }),
        ],
        values: vec![],
        functions: vec![],
    }
}

#[derive(Debug, Clone)]
pub struct ProgramTree {
    pub context: Arc<Mutex<RootContext>>,
    pub main: PtDocument,
}

pub fn create_program_tree(
    main_document: TreeNode,
    main_uri: String,
) -> Result<ProgramTree, PtError> {
    let TreeNode::Document(doc) = main_document else {
        return Err(PtError::in_at(main_uri, main_document.get_range(), "Expected document."));
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
