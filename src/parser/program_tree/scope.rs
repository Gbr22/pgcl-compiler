use super::{function_declaration::FunctionDeclarationReferable, type_declaration::TypeDeclarationReferable, value_declaration::ValueDeclarationReferable};

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct DocumentScopeId {
    pub uri: String,
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
pub enum ScopeId {
    Global,
    Function(FunctionScopeId),
    Document(DocumentScopeId),
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub types: Vec<TypeDeclarationReferable>,
    pub values: Vec<ValueDeclarationReferable>,
    pub functions: Vec<FunctionDeclarationReferable>,
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