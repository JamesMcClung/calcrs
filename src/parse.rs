mod error;
pub mod expr;
pub mod token;

use error::Error;
pub use expr::{Expression, Value};
use token::Token;

#[derive(Debug)]
enum Parse {
    Tok(Token),
    Expr(Expression),
}

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

fn parse_tokens(tokens: &[Token]) -> Result<Expression, Error> {
    let tokens = trim_spaces(tokens);
    let parse_seq = [try_parse_integer, try_parse_sum, try_parse_unary_plus, try_parse_unary_minus];
    for try_parse in parse_seq {
        if let Some(expr) = try_parse(tokens)? {
            return Ok(expr);
        }
    }
    Err(Error::SyntaxError(format!("{tokens:?}")))
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
    use std::fmt::Debug;
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn fail_test<T: Debug, U: Debug>(input: &str, expected_output: T, actual_output: U) {
        INIT.call_once(|| std::panic::set_hook(Box::new(error::minimal_panic_hook)));
        panic!("input:    \"{input}\"\nexpected: {expected_output:?}\nactual:   {actual_output:?}");
    }

    fn expect_value(expected_output: Value, input: &str) {
        match parse(input) {
            Err(err) => fail_test(input, expected_output, err),
            Ok(expr) => match expr.eval() {
                Ok(output) if output == expected_output => return,
                Ok(bad_output) => fail_test(input, expected_output, bad_output),
                Err(err) => fail_test(input, expected_output, err),
            },
        }
    }

    fn expect_syntax_error(input: &str) {
        match parse(input) {
            Err(Error::SyntaxError(_)) => return,
            Err(bad_err) => fail_test(input, "SyntaxError", bad_err),
            Ok(expr) => fail_test(input, "SyntaxError", expr),
        }
    }

    #[test]
    fn parse_int() {
        expect_value(Value::Integer(0), "0");
        expect_value(Value::Integer(11111), "11111");
        expect_value(Value::Integer(-32), "-32");
        expect_value(Value::Integer(234), "+234");
    }

    #[test]
    fn parse_sum() {
        expect_value(Value::Integer(5), "1+4");
        expect_value(Value::Integer(15), "-4+19");
        expect_value(Value::Integer(-19), "-20+1");
        expect_value(Value::Integer(3), "0+3");
        expect_value(Value::Integer(-3), "0+ -3");
        expect_value(Value::Integer(-8), "-6 + -2");
        expect_value(Value::Integer(8), "+6 + +2");
        expect_value(Value::Integer(6), "1 + 2 + 3");
        expect_value(Value::Integer(4), "-1 + 2 + 3");
        expect_value(Value::Integer(4), "1 + + 3");
        expect_value(Value::Integer(-2), "1 + - 3");
        expect_value(Value::Integer(-2), "1 + - - + - 3");
        expect_value(Value::Integer(4), "1 + - - + - - 3");
    }

    #[test]
    fn parse_unary_plus() {
        expect_value(Value::Integer(3), "+3");
        expect_value(Value::Integer(3), "+ +3");
        expect_value(Value::Integer(3), "+ + 3");
        expect_value(Value::Integer(-3), "+ + -3");
        expect_value(Value::Integer(-3), "+ + - 3");
        expect_syntax_error("++3");
    }

    #[test]
    fn parse_unary_minus() {
        expect_value(Value::Integer(-3), "-3");
        expect_value(Value::Integer(3), "- -3");
        expect_value(Value::Integer(3), "- - 3");
        expect_value(Value::Integer(3), "- - +3");
        expect_value(Value::Integer(3), "- - + 3");
        expect_syntax_error("--3");
    }
}
