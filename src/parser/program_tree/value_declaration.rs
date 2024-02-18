use crate::parser::nodes::var_declaration::PtVarDeclaration as Uniform;

use super::scope::Referable;

trait_enum! {
    #[derive(Debug, Clone)]
    pub enum ValueDeclarationReferable: ValueDeclarationReferableLike {
        Uniform
    }
}

pub trait ValueDeclarationReferableLike: Referable {
    
}
