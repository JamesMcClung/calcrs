pub mod expr;

pub use expr::{Expression, Value};

pub fn parse(expr: &str) -> Result<Expression, String> {
    return Err(String::from(expr));
}
