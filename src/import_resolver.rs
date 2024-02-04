use std::{fmt::Debug, sync::Arc};

pub struct ImportResolver {
    pub resolve: Arc<dyn Fn(&str) -> Option<String> + Sync + Send>
}

impl Debug for ImportResolver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImportResolver").field("resolve", &"<function>".to_owned()).finish()
    }
}

impl ImportResolver {
    pub fn resolve(&self, uri: &str) -> Option<String> {
        (*self.resolve)(uri)
    } 
}
