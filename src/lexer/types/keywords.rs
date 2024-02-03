pub static RETURN: &str = "return";
pub static UNIFORM: &str = "uniform";
pub static FN: &str = "fn";
pub static LET: &str = "let";
pub static MUT: &str = "mut";
pub static STRUCT: &str = "struct";
pub static CONST: &str = "const";

pub fn get_keywords() -> Vec<&'static str> {
    vec![RETURN, UNIFORM, FN, LET, MUT, STRUCT, CONST]
}
pub fn is_keyword(str: &str) -> bool {
    get_keywords().contains(&str)
}
