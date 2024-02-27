use crate::parser::nodes::types::{
    simple::global_type_ref,
    typ::{PtConcreteTypeExpression, PtTypeExpression},
};

use super::{function_declaration::FunctionDeclarationReferableLike, scope::Referable};

#[derive(Debug, Clone)]
pub struct NativeFunction {
    pub name: String,
    pub args: Vec<NativeFunctionArg>,
    pub return_type: PtConcreteTypeExpression,
}

impl Referable for NativeFunction {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl FunctionDeclarationReferableLike for NativeFunction {}

#[derive(Debug, Clone)]
pub struct NativeFunctionArg {
    pub name: String,
    pub typ: PtConcreteTypeExpression,
}

impl NativeFunctionArg {
    pub fn new(name: impl Into<String>, typ: impl Into<String>) -> NativeFunctionArg {
        NativeFunctionArg {
            name: name.into(),
            typ: global_type_ref(typ).into(),
        }
    }
}
