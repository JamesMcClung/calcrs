#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    WholeNumber(String),
    Operator(String),
    Space,
}

pub fn tokenize(expr: &str) -> Vec<Token> {
    Vec::new()
}
