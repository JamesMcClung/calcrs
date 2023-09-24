use super::Token;

#[derive(Debug)]
pub enum Error {
    SyntaxError(Vec<Token>),
    EvalError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Self::SyntaxError(tokens) => format!("SyntaxError: invalid syntax {tokens:?}"),
            Self::EvalError(s) => format!("EvalError: {s}"),
        })
    }
}
