use crate::common::range::Range;

use super::{
    function_declaration::FunctionDeclarationReferable, type_declaration::TypeDeclarationReferable,
    value_declaration::ValueDeclarationReferable,
};

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct DocumentScopeId {
    pub uri: String,
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct BlockScopedId {
    uri: String,
    range: Range,
}

impl BlockScopedId {
    pub fn new(uri: impl Into<String>, range: Range) -> BlockScopedId {
        BlockScopedId {
            uri: uri.into(),
            range,
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct FunctionScopeId {
    uri: String,
    name: String,
}

impl FunctionScopeId {
    pub fn new(uri: impl Into<String>, name: impl Into<String>) -> FunctionScopeId {
        FunctionScopeId {
            uri: uri.into(),
            name: name.into(),
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct VarScopeId {
    uri: String,
    name: String,
    range: Range,
}

impl VarScopeId {
    pub fn new(uri: impl Into<String>, name: impl Into<String>, range: Range) -> VarScopeId {
        VarScopeId {
            uri: uri.into(),
            name: name.into(),
            range,
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub enum ScopeId {
    Global,
    Function(FunctionScopeId),
    Document(DocumentScopeId),
    Block(BlockScopedId),
    Var(VarScopeId),
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub types: Vec<TypeDeclarationReferable>,
    pub values: Vec<ValueDeclarationReferable>,
    pub functions: Vec<FunctionDeclarationReferable>,
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            types: vec![],
            values: vec![],
            functions: vec![],
        }
    }
}

pub trait Referable {
    fn get_name(&self) -> &str;
}
