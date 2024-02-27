use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard, PoisonError},
};

use crate::{
    common::range::Range,
    parser::{
        nodes::{document::PtDocument, types::simple::global_type_ref},
        tree::{ParseError, TreeNode},
    },
};
use rayon::{iter::Either, prelude::*, vec::IntoIter};

use super::{
    function_declaration::FunctionDeclarationReferable,
    native_const::NativeConst,
    native_function::{NativeFunction, NativeFunctionArg},
    scope::{Scope, ScopeId},
    type_declaration::{PrimitiveTypeDeclaration, TypeDeclarationReferable},
    value::Value,
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
    pub fn message(message: impl Into<String>) -> PtError {
        PtError {
            uri: None,
            range: None,
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
        root_context: RootContextMutRef,
        context: &CurrentContext,
    ) -> Result<Final, PtError>;
}

pub fn try_map_into_pt<Source, Destination>(
    vec: Vec<Source>,
    root_context: RootContextMutRef,
    context: &CurrentContext,
) -> Result<Vec<Destination>, PtError>
where
    Destination: Sized + Sync + Send,
    Source: TryIntoPt<Destination> + Sized + Sync + Send,
{
    let iter: IntoIter<Source> = vec.into_par_iter();
    let vecs: (Vec<Destination>, Vec<PtError>) = iter.partition_map(|fun: Source| {
        let result: Result<Destination, PtError> = fun.try_into_pt(root_context.clone(), context);

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

pub type RootContextMutRefType = Arc<Mutex<RootContext>>;

#[derive(Debug, Clone)]
pub struct RootContextMutRef(RootContextMutRefType);

impl RootContextMutRef {
    pub fn insert_scope(&mut self, scope_id: ScopeId, scope: Scope) -> Result<(), PtError> {
        let mut root = self.0.lock().map_err(|_e| {
            PtError::message(format!(
                "Encountered poisoned mutex while trying to insert scope: {:?}",
                scope_id
            ))
        })?;
        root.scopes.insert(scope_id.clone(), scope);

        Ok(())
    }
}

pub type RootContextMutexPoisonedError<'a> = PoisonError<MutexGuard<'a, RootContext>>;

impl From<RootContextMutexPoisonedError<'_>> for PtError {
    fn from(_val: RootContextMutexPoisonedError<'_>) -> Self {
        PtError {
            uri: None,
            range: None,
            message: "Mutex poisoned".to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CurrentContext {
    pub uri: String,
    pub accessible_scopes: Vec<ScopeId>,
}

impl CurrentContext {
    pub fn extend(mut self, new_scope: ScopeId) -> Self {
        self.accessible_scopes.push(new_scope);

        self
    }
}

pub fn create_global_scope() -> Scope {
    Scope {
        types: vec![
            TypeDeclarationReferable::Primitive(PrimitiveTypeDeclaration {
                name: "f32".to_owned(),
                description: Some("32-bit floating point value".to_owned()),
            }),
            TypeDeclarationReferable::Primitive(PrimitiveTypeDeclaration {
                name: "i32".to_owned(),
                description: Some("32-bit signed integer".to_owned()),
            }),
            TypeDeclarationReferable::Primitive(PrimitiveTypeDeclaration {
                name: "u32".to_owned(),
                description: Some("32-bit unsigned integer".to_owned()),
            }),
            TypeDeclarationReferable::Primitive(PrimitiveTypeDeclaration {
                name: "bool".to_owned(),
                description: Some("boolean (true or false) value".to_owned()),
            }),
        ],
        values: vec![
            NativeConst::new("true", Value::Bool(true)).into(),
            NativeConst::new("false", Value::Bool(false)).into(),
            NativeConst::new("pi", Value::F32(std::f32::consts::PI)).into(),
        ],
        functions: vec![FunctionDeclarationReferable::NativeFunction(
            NativeFunction {
                name: "cos".to_owned(),
                return_type: global_type_ref("f32").into(),
                args: vec![NativeFunctionArg::new("value", "f32")],
            },
        )],
    }
}

#[derive(Debug, Clone)]
pub struct ProgramTree {
    pub context: RootContextMutRefType,
    pub main: PtDocument,
}

pub fn create_program_tree(
    main_document: TreeNode,
    main_uri: String,
) -> Result<ProgramTree, PtError> {
    let TreeNode::Document(doc) = main_document else {
        return Err(PtError::in_at(
            main_uri,
            main_document.get_range(),
            "Expected document.",
        ));
    };

    let mut scopes = HashMap::new();
    scopes.insert(ScopeId::Global, create_global_scope());

    let root_context = Arc::new(Mutex::new(RootContext { scopes }));

    let current_context = CurrentContext {
        uri: main_uri,
        accessible_scopes: vec![ScopeId::Global],
    };

    let main: PtDocument =
        doc.try_into_pt(RootContextMutRef(root_context.clone()), &current_context)?;

    Ok(ProgramTree {
        context: root_context,
        main,
    })
}
