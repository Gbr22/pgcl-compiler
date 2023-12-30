
#[macro_export]
macro_rules! pop_front_node {
    ($list:ident, $msg:expr, $name:pat, $condition:expr) => {
        let $name = $list.pop_front() else {
            return crate::parser::tree::ParseError::at($list.range, $msg).into();
        };
        if !($condition) {
            return crate::parser::tree::ParseError::at($list.range, $msg).into();
        }
    };
}