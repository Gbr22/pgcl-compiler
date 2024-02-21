use crate::parser::nodes::function_declaration::PtFunctionDeclaration as UserFunction;
use crate::parser::program_tree::native_function::NativeFunction;

use super::scope::Referable;

trait_enum! {
    #[derive(Debug, Clone)]
    pub enum FunctionDeclarationReferable: FunctionDeclarationReferableLike {
        UserFunction,
        NativeFunction
    }
}

pub trait FunctionDeclarationReferableLike: Referable {}
