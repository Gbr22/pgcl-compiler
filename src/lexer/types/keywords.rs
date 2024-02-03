pub static RETURN: &'static str = "return";
pub static UNIFORM: &'static str = "uniform";
pub static FN: &'static str = "fn";
pub static LET: &'static str = "let";
pub static MUT: &'static str = "mut";
pub static STRUCT: &'static str = "struct";
pub static CONST: &'static str = "const";

pub fn get_keywords() -> Vec<&'static str> {
    vec![RETURN, UNIFORM, FN, LET, MUT, STRUCT, CONST]
}
pub fn is_keyword(str: &str) -> bool {
    get_keywords().contains(&str)
}
