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

pub trait Referable {
    fn get_name(&self) -> &str;
}