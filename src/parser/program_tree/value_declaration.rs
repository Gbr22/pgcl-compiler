use crate::parser::nodes::uniform_declaration::PtUniformDeclaration as Uniform;

use super::scope::Referable;

trait_enum! {
    #[derive(Debug, Clone)]
    pub enum ValueDeclarationReferable: ValueDeclarationReferableLike {
        Uniform
    }
}

pub trait ValueDeclarationReferableLike: Referable {
    
}
