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
    let parse_seq = [try_parse_integer, try_parse_sum, try_parse_unary_plus, try_parse_unary_minus];
    for try_parse in parse_seq {
        if let Some(expr) = try_parse(tokens)? {
            return Ok(expr);
        }
    }
    Err(Error::SyntaxError(tokens.to_vec()))
}

fn try_parse_integer(tokens: &[Token]) -> Result<Option<Expression>, Error> {
    Ok(match tokens {
        [Token::WholeNumber(num)] => Some(Expression::Constant(Value::Integer(num.parse().expect("num should always be a sequence of digits")))),
        _ => None,
    })
}

fn try_parse_unary_plus(tokens: &[Token]) -> Result<Option<Expression>, Error> {
    Ok(match tokens {
        [_] => None,
        [_, Token::Operator(op), ..] if op == "+" || op == "-" => None,
        [Token::Operator(op), rest @ ..] if op == "+" => Some(Expression::UnaryPlus(Box::new(parse_tokens(rest)?))),
        _ => None,
    })
}

fn try_parse_unary_minus(tokens: &[Token]) -> Result<Option<Expression>, Error> {
    Ok(match tokens {
        [_] => None,
        [_, Token::Operator(op), ..] if op == "+" || op == "-" => None,
        [Token::Operator(op), rest @ ..] if op == "-" => Some(Expression::UnaryMinus(Box::new(parse_tokens(rest)?))),
        _ => None,
    })
}

fn try_parse_sum(tokens: &[Token]) -> Result<Option<Expression>, Error> {
    if tokens.len() < 3 {
        return Ok(None);
    }
    let mut found_lhs = false;
    for i in 1..(tokens.len() - 1) {
        if matches!(tokens[i - 1], Token::Identifier(_) | Token::WholeNumber(_)) {
            found_lhs = true;
        }
        if !found_lhs {
            continue;
        }
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
        assert!(matches!(parse("1 + + 3")?.eval()?, Value::Integer(4)));
        assert!(matches!(parse("1 + - 3")?.eval()?, Value::Integer(-2)));
        assert!(matches!(parse("1 + - - + - 3")?.eval()?, Value::Integer(-2)));
        assert!(matches!(parse("1 + - - + - - 3")?.eval()?, Value::Integer(4)));
        Ok(())
    }

    #[test]
    fn parse_unary_plus() -> Result<(), Error> {
        assert!(matches!(parse("+3")?.eval()?, Value::Integer(3)));
        assert!(matches!(parse("+ +3")?.eval()?, Value::Integer(3)));
        assert!(matches!(parse("+ + 3")?.eval()?, Value::Integer(3)));
        assert!(matches!(parse("+ + -3")?.eval()?, Value::Integer(-3)));
        assert!(matches!(parse("+ + - 3")?.eval()?, Value::Integer(-3)));
        assert!(matches!(parse("++3"), Err(Error::SyntaxError(_))));
        Ok(())
    }

    #[test]
    fn parse_unary_minus() -> Result<(), Error> {
        assert!(matches!(parse("-3")?.eval()?, Value::Integer(-3)));
        assert!(matches!(parse("- -3")?.eval()?, Value::Integer(3)));
        assert!(matches!(parse("- - 3")?.eval()?, Value::Integer(3)));
        assert!(matches!(parse("- - +3")?.eval()?, Value::Integer(3)));
        assert!(matches!(parse("- - + 3")?.eval()?, Value::Integer(3)));
        assert!(matches!(parse("--3"), Err(Error::SyntaxError(_))));
        Ok(())
    }
}
