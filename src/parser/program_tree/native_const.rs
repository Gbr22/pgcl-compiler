use crate::parser::nodes::types::{internal::global_type_ref, typ::PtType};

use super::{scope::Referable, value::Value, value_declaration::{ValueDeclarationReferable, ValueDeclarationReferableLike}};

#[derive(Debug, Clone)]
pub struct NativeConst {
    pub name: String,
    pub value: Value
}

impl NativeConst {
    pub fn new(name: impl Into<String>, value: Value) -> NativeConst {
        NativeConst {
            name: name.into(),
            value,
        }
    }
}

impl Into<ValueDeclarationReferable> for NativeConst {
    fn into(self) -> ValueDeclarationReferable {
        ValueDeclarationReferable::NativeConst(self)
    }
}

impl Referable for NativeConst {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl ValueDeclarationReferableLike for NativeConst {
    fn get_type(&self) -> PtType {
        self.value.get_type()
    }
}