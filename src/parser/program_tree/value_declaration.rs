use crate::parser::nodes::{function_arg::PtFunctionArg, types::typ::PtType, var_declaration::PtVarDeclaration};

use super::{native_const::NativeConst, scope::Referable};

#[derive(Debug, Clone)]
pub enum ValueDeclarationReferable {
    Var(PtVarDeclaration),
    NativeConst(NativeConst),
    FunctionArg(PtFunctionArg)
}

impl Referable for ValueDeclarationReferable {
    fn get_name(&self) -> &str {
        match self {
            ValueDeclarationReferable::Var(v) => v.get_name(),
            ValueDeclarationReferable::NativeConst(n) => n.get_name(),
            ValueDeclarationReferable::FunctionArg(arg) => arg.get_name()
        }
    }
}

impl ValueDeclarationReferableLike for ValueDeclarationReferable {
    fn get_type(&self) -> PtType {
        match self {
            ValueDeclarationReferable::Var(v) => v.get_type(),
            ValueDeclarationReferable::NativeConst(nc) => nc.get_type(),
            ValueDeclarationReferable::FunctionArg(arg) => arg.get_type(),
        }
    }
}


pub trait ValueDeclarationReferableLike: Referable {
    fn get_type(&self) -> PtType;
}
