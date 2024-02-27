use crate::parser::nodes::types::{
    simple::global_type_ref,
    typ::{PtConcreteTypeExpression, PtTypeExpression},
};

#[derive(Debug, Clone)]
pub enum Value {
    F32(f32),
    I32(i32),
    Bool(bool),
}

impl Value {
    pub fn get_type(&self) -> PtTypeExpression {
        let str = match self {
            Value::F32(_) => "f32",
            Value::I32(_) => "i32",
            Value::Bool(_) => "bool",
        };

        let conrete: PtConcreteTypeExpression = global_type_ref(str).into();

        conrete.into()
    }
    pub fn string_value(&self) -> String {
        match self {
            Value::F32(f) => format!("{}", f),
            Value::I32(i) => format!("{}", i),
            Value::Bool(b) => format!("{}", b),
        }
    }
}
