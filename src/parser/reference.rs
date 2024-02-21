use std::{cell::Ref, sync::{Arc, Mutex}};

use super::program_tree::{program_tree::RootContext, scope::{Referable, ScopeId}, type_declaration::TypeDeclarationReferable, value_declaration::ValueDeclarationReferable};

#[derive(Debug, Clone)]
pub struct Reference {
    pub scopes: Vec<ScopeId>,
    pub name: String
}

impl Reference {
    pub fn new(name: impl Into<String>, scopes: Vec<ScopeId>) -> Reference {
        Reference { scopes, name: name.into() }
    }
}

#[derive(Debug, Clone)]
pub struct TypeReference(pub Reference);

impl TypeReference {
    pub fn resolve(&self, root: &RootContext) -> Option<TypeDeclarationReferable> {
        let r = &self.0;
        for id in r.scopes.iter().rev() {
            let Some(scope) = root.scopes.get(id) else {
                continue;
            };
            for t in scope.types.iter() {
                if t.get_name() == r.name {
                    return Some(t.clone());
                }
            }
        };

        None
    }
}

#[derive(Debug, Clone)]
pub struct ValueReference(pub Reference);

impl ValueReference {
    pub fn resolve(&self, root: &RootContext) -> Option<ValueDeclarationReferable> {
        let r = &self.0;
        for id in r.scopes.iter().rev() {
            let Some(scope) = root.scopes.get(id) else {
                continue;
            };
            for t in scope.values.iter() {
                if t.get_name() == r.name {
                    return Some(t.clone());
                }
            }
        };

        None
    }
}