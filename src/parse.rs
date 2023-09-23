pub mod expr;
pub mod token;

pub use expr::{Expression, Value};
use token::Token;

pub fn parse(expr: &str) -> Result<Expression, String> {
    let tokens = token::tokenize(expr);
    if let Some(expr) = try_parse_integer(&tokens) {
        Ok(expr)
    } else {
        Err(String::from(expr))
    }
}

fn try_parse_integer(tokens: &Vec<Token>) -> Option<Expression> {
    match &tokens[..] {
        [Token::WholeNumber(num)] => Some(Expression::Constant(Value::Integer(num.parse().unwrap()))),
        [Token::Operator(op), Token::WholeNumber(num)] if op == "+" => Some(Expression::Constant(Value::Integer(num.parse().unwrap()))),
        [Token::Operator(op), Token::WholeNumber(num)] if op == "-" => Some(Expression::Constant(Value::Integer(-num.parse::<i64>().unwrap()))),
        _ => None,
    }
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

    #[test]
    fn parse_sum() -> Result<(), String> {
        assert!(matches!(parse("1+4")?.eval()?, Value::Integer(5)));
        assert!(matches!(parse("-4+19")?.eval()?, Value::Integer(15)));
        assert!(matches!(parse("-20+1")?.eval()?, Value::Integer(-19)));
        assert!(matches!(parse("0+3")?.eval()?, Value::Integer(3)));
        assert!(matches!(parse("0+ -3")?.eval()?, Value::Integer(-3)));
        assert!(matches!(parse("-6 + -2")?.eval()?, Value::Integer(-8)));
        assert!(matches!(parse("+6 + +2")?.eval()?, Value::Integer(8)));
        Ok(())
    }
}
