use crate::parser::nodes::types::{internal::global_type_ref, typ::PtType};

#[derive(Debug, Clone)]
pub enum Value {
    F32(f32),
    I32(i32),
    Bool(bool)
}

impl Value {
    pub fn get_type(&self) -> PtType {
        let str = match self {
            Value::F32(_) => "f32",
            Value::I32(_) => "i32",
            Value::Bool(_) => "bool",
        };

        global_type_ref(str).into()
    }
}