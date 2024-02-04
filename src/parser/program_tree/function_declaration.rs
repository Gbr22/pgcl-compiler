use crate::parser::nodes::function_declaration::PtFunctionDeclaration as UserFunction;

use super::scope::Referable;

trait_enum! {
    #[derive(Debug, Clone)]
    pub enum FunctionDeclarationReferable: FunctionDeclarationReferableLike {
        UserFunction
    }
}

pub trait FunctionDeclarationReferableLike: Referable {
    
}