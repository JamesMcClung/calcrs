#[derive(Debug)]
pub enum Error {
    TokenizeError(char),
    SyntaxError(String),
    EvalError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Self::TokenizeError(c) => format!("TokenizeError: unknown character '{c}'"),
            Self::SyntaxError(s) => format!("SyntaxError: {s}"),
            Self::EvalError(s) => format!("EvalError: {s}"),
        })
    }
}

#[cfg(test)]
pub fn minimal_panic_hook(info: &std::panic::PanicInfo) {
    if let Some(msg) = info.payload().downcast_ref::<&str>() {
        println!("{msg}");
    } else if let Some(msg) = info.payload().downcast_ref::<String>() {
        println!("{msg}");
    } else {
        println!("unable to print panic message");
    }
}
