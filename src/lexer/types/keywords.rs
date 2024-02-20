pub static ALIAS: &str = "alias";
pub static BREAK: &str = "break";
pub static CASE: &str = "case";
pub static CONST: &str = "const";
pub static CONST_ASSERT: &str = "const_assert";
pub static CONTINUE: &str = "continue";
pub static CONTINUING: &str = "continuing";
pub static DEFAULT: &str = "default";
pub static DIAGNOSTIC: &str = "diagnostic";
pub static DISCARD: &str = "discard";
pub static ELSE: &str = "else";
pub static ENABLE: &str = "enable";
pub static FN: &str = "fn";
pub static FOR: &str = "for";
pub static IF: &str = "if";
pub static LET: &str = "let";
pub static LOOP: &str = "loop";
pub static OVERRIDE: &str = "override";
pub static REQUIRES: &str = "requires";
pub static RETURN: &str = "return";
pub static STRUCT: &str = "struct";
pub static SWITCH: &str = "switch";
pub static VAR: &str = "var";
pub static WHILE: &str = "while";

pub fn get_keywords() -> Vec<&'static str> {
    vec![ALIAS, BREAK, CASE, CONST, CONST_ASSERT, CONTINUE, CONTINUING, DEFAULT, DIAGNOSTIC, DISCARD, ELSE, ENABLE, FN, FOR, IF, LET, LOOP, OVERRIDE, REQUIRES, RETURN, STRUCT, SWITCH, VAR, WHILE]
}
pub fn is_keyword(str: &str) -> bool {
    get_keywords().contains(&str)
}
