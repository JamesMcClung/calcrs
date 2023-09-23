pub mod expr;

pub use expr::{Expression, Value};

pub fn parse(expr: &str) -> Result<Expression, String> {
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
