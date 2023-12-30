
#[macro_export]
macro_rules! pop_front_node {
    ($list:ident, $msg:expr, $patten:pat, $condition:expr) => {
        let pop_front_internal_value = $list.pop_front_internal();
        let $patten = pop_front_internal_value.0 else {
            return crate::parser::tree::ParseError::at(pop_front_internal_value.1, $msg).into();
        };
        if !($condition) {
            return crate::parser::tree::ParseError::at(pop_front_internal_value.1, $msg).into();
        }
    };
}

#[macro_export]
macro_rules! pop_back_node {
    ($list:ident, $msg:expr, $patten:pat, $condition:expr) => {
        let pop_front_internal_value = $list.pop_back_internal();
        let $patten = pop_front_internal_value.0 else {
            return crate::parser::tree::ParseError::at(pop_front_internal_value.1, $msg).into();
        };
        if !($condition) {
            return crate::parser::tree::ParseError::at(pop_front_internal_value.1, $msg).into();
        }
    };
}