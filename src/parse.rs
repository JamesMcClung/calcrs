pub mod expr;
pub mod token;

pub use expr::{Expression, Value};
use once_cell::sync::Lazy;
use regex::Regex;

static REGEX_INTEGER: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(?:\+|-)?\d+$").unwrap());

pub fn parse(expr: &str) -> Result<Expression, String> {
    if REGEX_INTEGER.is_match(expr) {
        let val = Value::Integer(expr.parse().unwrap());
        return Ok(Expression::Constant(val));
    }
    return Err(String::from(expr));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_int() -> Result<(), String> {
        assert!(matches!(parse("0")?.eval()?, Value::Integer(0)));
        assert!(matches!(parse("11111")?.eval()?, Value::Integer(11111)));
        assert!(matches!(parse("-32")?.eval()?, Value::Integer(-32)));
        assert!(matches!(parse("+234")?.eval()?, Value::Integer(234)));
        Ok(())
    }
}
