use enum_dispatch::enum_dispatch;

use crate::parser::nodes::{
    function_arg::PtFunctionArg, types::typ::PtTypeExpression, var_declaration::PtVarDeclaration,
};

use super::{native_const::NativeConst, scope::Referable};

#[derive(Debug, Clone)]
#[enum_dispatch]
pub enum ValueDeclarationReferable {
    Var(PtVarDeclaration),
    NativeConst(NativeConst),
    FunctionArg(PtFunctionArg),
}

impl Referable for ValueDeclarationReferable {
    fn get_name(&self) -> &str {
        match self {
            ValueDeclarationReferable::Var(v) => v.get_name(),
            ValueDeclarationReferable::NativeConst(n) => n.get_name(),
            ValueDeclarationReferable::FunctionArg(arg) => arg.get_name(),
        }
    }
}

#[enum_dispatch(ValueDeclarationReferable)]
pub trait ValueDeclarationReferableLike: Referable {
    fn get_type(&self) -> PtTypeExpression;
}
