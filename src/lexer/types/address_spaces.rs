pub static FUNCTION: &str = "function";
pub static PRIVATE: &str = "private";
pub static WORKGROUP: &str = "workgroup";
pub static UNIFORM: &str = "uniform";
pub static STORAGE: &str = "storage";
pub static HANDLE: &str = "handle";

pub fn get_address_spaces() -> Vec<&'static str> {
    vec![FUNCTION, PRIVATE, WORKGROUP, UNIFORM, STORAGE, HANDLE]
}
pub fn is_address_space(str: &str) -> bool {
    get_address_spaces().contains(&str)
}
