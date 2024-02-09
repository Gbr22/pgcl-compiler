use std::{cell::Ref, sync::{Arc, Mutex}};

use super::program_tree::{program_tree::RootContext, scope::{Referable, ScopeId}, type_declaration::TypeDeclarationReferable};

#[derive(Debug, Clone)]
pub struct Reference {
    pub root: Arc<Mutex<RootContext>>,
    pub scopes: Vec<ScopeId>,
    pub name: String
}

#[derive(Debug, Clone)]
pub struct TypeReference(pub Reference);

impl TypeReference {
    pub fn resolve(&self) -> Option<TypeDeclarationReferable> {
        let r = &self.0;
        let root = r.root.lock().unwrap();
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