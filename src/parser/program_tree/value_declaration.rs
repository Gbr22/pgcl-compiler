use crate::parser::nodes::var_declaration::PtVarDeclaration;

use super::{native_const::NativeConst, scope::Referable};

#[derive(Debug, Clone)]
pub enum ValueDeclarationReferable {
    Var(PtVarDeclaration),
    NativeConst(NativeConst)
}

impl Referable for ValueDeclarationReferable {
    fn get_name(&self) -> &str {
        match self {
            ValueDeclarationReferable::Var(v) => v.get_name(),
            ValueDeclarationReferable::NativeConst(n) => n.get_name(),
        }
    }
}

impl ValueDeclarationReferableLike for ValueDeclarationReferable {

}


pub trait ValueDeclarationReferableLike: Referable {
    
}
