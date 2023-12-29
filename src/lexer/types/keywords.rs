pub fn get_keywords() -> Vec<&'static str> {
    vec![
        "let",
        "mut",
        "struct",
        "fn",
        "uniform",
        "return"
    ]
}
pub fn is_keyword(str: &str) -> bool {
    get_keywords().contains(&str)
}