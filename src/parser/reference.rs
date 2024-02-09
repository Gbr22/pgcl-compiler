use std::sync::{Arc, Mutex};

use super::program_tree::{program_tree::RootContext, scope::ScopeId};

#[derive(Debug, Clone)]
pub struct Reference {
    pub root: Arc<Mutex<RootContext>>,
    pub scopes: Vec<ScopeId>,
    pub name: String
}