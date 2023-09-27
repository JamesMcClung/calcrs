mod error;
pub mod expr;
pub mod token;

use error::Error;
pub use expr::{Expression, Value};
use token::Token;

pub fn parse(expr: &str) -> Result<Expression, Error> {
    parse_tokens(&token::tokenize(expr)?)
}

fn trim_spaces(mut tokens: &[Token]) -> &[Token] {
    if tokens.starts_with(&[Token::Space]) {
        tokens = &tokens[1..];
    }
    if tokens.ends_with(&[Token::Space]) {
        tokens = &tokens[..tokens.len() - 1];
    }
    tokens
}

pub fn parse_tokens(tokens: &[Token]) -> Result<Expression, Error> {
    let tokens = trim_spaces(tokens);

    if let Some(expr) = try_parse_integer(tokens)? {
        Ok(expr)
    } else if let Some(expr) = try_parse_sum(tokens)? {
        Ok(expr)
    } else {
        Err(Error::SyntaxError(tokens.to_vec()))
    }
}

fn try_parse_integer(tokens: &[Token]) -> Result<Option<Expression>, Error> {
    let parse_num = |num: &str| num.parse::<i64>().expect("num should always be a sequence of digits");
    Ok(match tokens {
        [Token::WholeNumber(num)] => Some(Expression::Constant(Value::Integer(parse_num(num)))),
        [Token::Operator(op), Token::WholeNumber(num)] if op == "+" => Some(Expression::Constant(Value::Integer(parse_num(num)))),
        [Token::Operator(op), Token::WholeNumber(num)] if op == "-" => Some(Expression::Constant(Value::Integer(-parse_num(num)))),
        _ => None,
    })
}

fn try_parse_sum(tokens: &[Token]) -> Result<Option<Expression>, Error> {
    if tokens.len() < 3 {
        return Ok(None);
    }
    for i in 1..(tokens.len() - 1) {
        match &tokens[i - 1..=i + 1] {
            [Token::Operator(_), _, _] => (),
            [_, _, Token::Operator(_)] => (),
            [_, Token::Operator(op), _] if op == "+" => return Ok(Some(Expression::Sum(Box::new(parse_tokens(&tokens[..i])?), Box::new(parse_tokens(&tokens[i + 1..])?)))),
            _ => (),
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_int() -> Result<(), Error> {
        assert!(matches!(parse("0")?.eval()?, Value::Integer(0)));
        assert!(matches!(parse("11111")?.eval()?, Value::Integer(11111)));
        assert!(matches!(parse("-32")?.eval()?, Value::Integer(-32)));
        assert!(matches!(parse("+234")?.eval()?, Value::Integer(234)));
        Ok(())
    }

    #[test]
    fn parse_sum() -> Result<(), Error> {
        assert!(matches!(parse("1+4")?.eval()?, Value::Integer(5)));
        assert!(matches!(parse("-4+19")?.eval()?, Value::Integer(15)));
        assert!(matches!(parse("-20+1")?.eval()?, Value::Integer(-19)));
        assert!(matches!(parse("0+3")?.eval()?, Value::Integer(3)));
        assert!(matches!(parse("0+ -3")?.eval()?, Value::Integer(-3)));
        assert!(matches!(parse("-6 + -2")?.eval()?, Value::Integer(-8)));
        assert!(matches!(parse("+6 + +2")?.eval()?, Value::Integer(8)));
        assert!(matches!(parse("1 + 2 + 3")?.eval()?, Value::Integer(6)));
        assert!(matches!(parse("-1 + 2 + 3")?.eval()?, Value::Integer(4)));
        assert!(matches!(parse("1 + + 3"), Err(Error::SyntaxError(_))));
        Ok(())
    }
}
