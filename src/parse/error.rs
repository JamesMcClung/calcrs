use super::Token;

#[derive(Debug)]
pub enum Error {
    TokenizeError(char),
    SyntaxError(Vec<Token>),
    EvalError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Self::TokenizeError(c) => format!("TokenizeError: unknown character '{c}'"),
            Self::SyntaxError(tokens) => format!("SyntaxError: invalid syntax {tokens:?}"),
            Self::EvalError(s) => format!("EvalError: {s}"),
        })
    }
}
