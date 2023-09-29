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
    Temp,
}

pub fn parse(expr: &str) -> Result<Expression, Error> {
    let mut tokens = token::tokenize(expr)?.into_iter().map(|tok| Parse::Tok(tok)).collect::<Vec<_>>();
    parse_whole_numbers(&mut tokens);
    parse_unary_ops(&mut tokens);
    parse_sums(&mut tokens);
    trim_spaces(&mut tokens);
    if tokens.len() == 1 {
        if let Parse::Expr(expr) = tokens.pop().expect("just checked size") {
            return Ok(expr);
        }
    }
    Err(Error::SyntaxError(format!("{tokens:?}")))
}

fn trim_spaces(tokens: &mut Vec<Parse>) {
    if matches!(tokens.last(), Some(Parse::Tok(Token::Space))) {
        tokens.pop();
    }
    if matches!(tokens.first(), Some(Parse::Tok(Token::Space))) {
        tokens.remove(0);
    }
}

fn parse_whole_numbers(tokens: &mut Vec<Parse>) {
    for tok in tokens {
        if let Parse::Tok(Token::WholeNumber(num)) = tok {
            let num = num.parse().expect("num should always be a sequence of digits");
            *tok = Parse::Expr(Expression::Constant(Value::Integer(num)))
        }
    }
}

fn parse_unary_ops(tokens: &mut Vec<Parse>) {
    let mut expr_idx = None;
    for i in (0..tokens.len()).rev() {
        match (&tokens[i], expr_idx) {
            (Parse::Expr(_), _) => expr_idx = Some(i),
            (Parse::Tok(Token::Operator(op)), Some(expri)) => {
                if !(op == "+" || op == "-") {
                    expr_idx = None;
                    continue;
                }
                match &tokens[usize::saturating_sub(i, 2)..i] {
                    [_, Parse::Tok(Token::Operator(_))] | [Parse::Tok(Token::Operator(_))] => expr_idx = None,
                    [] | [Parse::Tok(Token::Space)] | [Parse::Tok(Token::Operator(_)), Parse::Tok(Token::Space)] => {
                        let mut removed = tokens.splice(i..=expri, [Parse::Temp]).filter(|tok| !matches!(tok, Parse::Tok(Token::Space)));
                        let op = removed.next().expect("splice should have exactly 2 elements");
                        let expr = removed.next().expect("splice should have exactly 2 elements");
                        debug_assert!(removed.next().is_none(), "splice should have exactly 2 elements");
                        drop(removed);

                        if let (Parse::Tok(Token::Operator(op)), Parse::Expr(expr)) = (op, expr) {
                            let expr = Box::new(expr);
                            tokens[i] = Parse::Expr(if op == "+" { Expression::UnaryPlus(expr) } else { Expression::UnaryMinus(expr) })
                        } else {
                            panic!("splice should have an op and an expr");
                        }
                        expr_idx = Some(i);
                    },
                    _ => (),
                }
            },
            _ => (),
        }
    }
}

fn parse_sums(tokens: &mut Vec<Parse>) {
    let mut lhs_idx = None;
    let mut found_op = false;
    let mut i = 0;
    while i < tokens.len() {
        match (&tokens[i], lhs_idx, found_op) {
            (Parse::Expr { .. }, _, false) => lhs_idx = Some(i),
            (Parse::Tok(Token::Operator(op)), Some(_), false) => {
                if op == "+" {
                    found_op = true;
                } else {
                    lhs_idx = None;
                }
            },
            (Parse::Tok(Token::Operator(_)), Some(_), true) => {
                found_op = false;
                lhs_idx = None;
            },
            (Parse::Expr { .. }, Some(lhsi), true) => {
                let mut removed = tokens.splice(lhsi..=i, [Parse::Temp]).filter(|tok| !matches!(tok, Parse::Tok(Token::Space)));
                let lhs = removed.next().expect("splice should have exactly 3 elements");
                let op = removed.next().expect("splice should have exactly 3 elements");
                let rhs = removed.next().expect("splice should have exactly 3 elements");
                debug_assert!(removed.next().is_none(), "splice should have exactly 3 elements");
                drop(removed);

                if let (Parse::Expr(lhs_expr), Parse::Tok(Token::Operator(_)), Parse::Expr(rhs_expr)) = (lhs, op, rhs) {
                    let lhs = Box::new(lhs_expr);
                    let rhs = Box::new(rhs_expr);
                    tokens[lhsi] = Parse::Expr(Expression::Sum(lhs, rhs));
                    i = lhsi;
                } else {
                    panic!("splice should have an expr, op, and expr");
                }

                found_op = false;
            },
            _ => (),
        }
        i += 1;
    }
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
    fn syntax_errors() {
        expect_syntax_error("1++");
    }

    #[test]
    fn parse_int() {
        expect_value(Value::Integer(0), "0");
        expect_value(Value::Integer(0), " 0 ");
        expect_value(Value::Integer(11111), "11111");
        expect_value(Value::Integer(-32), "-32");
        expect_value(Value::Integer(234), "+234");
    }

    #[test]
    fn parse_sum() {
        expect_value(Value::Integer(5), "1+4");
        expect_value(Value::Integer(5), " 1 + 4 ");
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
        expect_syntax_error("1++3");
    }

    #[test]
    fn parse_unary_plus() {
        expect_value(Value::Integer(3), "+3");
        expect_value(Value::Integer(3), " + 3 ");
        expect_value(Value::Integer(3), "+ +3");
        expect_value(Value::Integer(3), "+ + 3");
        expect_value(Value::Integer(-3), "+ + -3");
        expect_value(Value::Integer(-3), "+ + - 3");
        expect_syntax_error("++3");
        expect_syntax_error("+ ++3");
    }

    #[test]
    fn parse_unary_minus() {
        expect_value(Value::Integer(-3), "-3");
        expect_value(Value::Integer(-3), " - 3 ");
        expect_value(Value::Integer(3), "- -3");
        expect_value(Value::Integer(3), "- - 3");
        expect_value(Value::Integer(3), "- - +3");
        expect_value(Value::Integer(3), "- - + 3");
        expect_syntax_error("- --3");
    }
}
