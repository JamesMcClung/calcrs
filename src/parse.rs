mod error;
pub mod expr;
pub mod token;

use error::Error;
pub use expr::{Expression, Value};
use let_match::let_match;
use token::Token;

#[derive(Debug)]
enum Parse {
    Tok(Token),
    Expr(Expression),
    Temp,
}

pub fn parse(expr: &str) -> Result<Expression, Error> {
    parse_impl(token::tokenize(expr)?.into_iter().map(|tok| Parse::Tok(tok)).collect::<Vec<_>>())
}

fn parse_impl(mut tokens: Vec<Parse>) -> Result<Expression, Error> {
    parse_parens(&mut tokens)?;
    parse_whole_numbers(&mut tokens);
    parse_unary_ops(&mut tokens);
    parse_sums_differences(&mut tokens);
    trim_spaces(&mut tokens);
    get_result(tokens)
}

fn get_result(mut tokens: Vec<Parse>) -> Result<Expression, Error> {
    if tokens.len() == 1 {
        if let Parse::Expr(expr) = tokens.pop().expect("just checked size") {
            return Ok(expr);
        }
    }
    let mut message = None;
    for i in 0..=tokens.len() {
        match (tokens.get(i), &mut message) {
            (Some(Parse::Temp), _) => panic!("Temps aren't allowed to persist"),
            (Some(Parse::Tok(tok)), None) => message = Some(tok.to_str().to_string()),
            (Some(Parse::Tok(tok)), Some(message)) => message.push_str(tok.to_str()),
            (Some(Parse::Expr(_)) | None, Some(message)) => return Err(Error::SyntaxError(format!("invalid syntax \"{}\"", message.trim()))),
            _ => (),
        }
    }
    unreachable!();
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

fn parse_parens(tokens: &mut Vec<Parse>) -> Result<(), Error> {
    let mut closing_paren_idx = None;
    let mut n_nested_parens = 0u8;
    for i in (0..tokens.len()).rev() {
        match (&tokens[i], closing_paren_idx) {
            (Parse::Tok(Token::Operator(op)), None) if op == ")" => closing_paren_idx = Some(i),
            (Parse::Tok(Token::Operator(op)), None) if op == "(" => return Err(Error::SyntaxError(String::from("unmatched \"(\""))),
            (Parse::Tok(Token::Operator(op)), Some(_)) if op == ")" => n_nested_parens += 1,
            (Parse::Tok(Token::Operator(op)), Some(_)) if op == "(" && n_nested_parens > 0 => n_nested_parens -= 1,
            (Parse::Tok(Token::Operator(op)), Some(closei)) if op == "(" && n_nested_parens == 0 => {
                let mut removed = tokens.splice(i..=closei, [Parse::Temp]).skip(1).collect::<Vec<_>>();
                removed.pop();
                trim_spaces(&mut removed);
                if removed.len() == 0 {
                    return Err(Error::SyntaxError(String::from("empty parentheses")));
                }
                tokens[i] = Parse::Expr(parse_impl(removed)?);
                closing_paren_idx = None;
            },
            _ => (),
        }
    }
    if closing_paren_idx.is_some() {
        return Err(Error::SyntaxError(String::from("unmatched \")\"")));
    }
    Ok(())
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
                        let_match!(Some(Parse::Tok(Token::Operator(op))) = removed.next());
                        let_match!(Some(Parse::Expr(expr)) = removed.next());
                        debug_assert!(removed.next().is_none(), "splice should have exactly 2 elements");
                        drop(removed);

                        let expr = Box::new(expr);
                        tokens[i] = Parse::Expr(if op == "+" { Expression::UnaryPlus(expr) } else { Expression::UnaryMinus(expr) });
                        expr_idx = Some(i);
                    },
                    _ => (),
                }
            },
            _ => (),
        }
    }
}

fn parse_sums_differences(tokens: &mut Vec<Parse>) {
    let mut lhs_idx = None;
    let mut found_op = false;
    let mut i = 0;
    while i < tokens.len() {
        match (&tokens[i], lhs_idx, found_op) {
            (Parse::Expr { .. }, _, false) => lhs_idx = Some(i),
            (Parse::Tok(Token::Operator(op)), Some(_), false) => {
                if op == "+" || op == "-" {
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
                let_match!(Some(Parse::Expr(lhs_expr)) = removed.next());
                let_match!(Some(Parse::Tok(Token::Operator(op))) = removed.next());
                let_match!(Some(Parse::Expr(rhs_expr)) = removed.next());
                debug_assert!(removed.next().is_none(), "splice should have exactly 3 elements");
                drop(removed);

                let lhs = Box::new(lhs_expr);
                let rhs = Box::new(rhs_expr);
                tokens[lhsi] = Parse::Expr(if op == "+" { Expression::Sum(lhs, rhs) } else { Expression::Difference(lhs, rhs) });

                i = lhsi;
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
        expect_syntax_error("()");
        expect_syntax_error("( )");
        expect_syntax_error("(() ( ) )");
    }

    #[test]
    fn parse_parens() {
        expect_value(Value::Integer(1), "(1)");
        expect_value(Value::Integer(1), " ( 1 ) ");
        expect_value(Value::Integer(1), "(( ( 1 ) ) ) ");
        expect_value(Value::Integer(2), "(1) + (1)");
        expect_value(Value::Integer(0), "(1) - (1)");
        expect_value(Value::Integer(-2), "(1) - (1 + 2)");
        expect_value(Value::Integer(-3), "-(1 + 2)");
        expect_value(Value::Integer(16), "(10+2)-(-5 + (3 - 2))");
        expect_syntax_error("1)");
        expect_syntax_error(" 1 ) ");
        expect_syntax_error("(1");
        expect_syntax_error(" ( 1 ");
        expect_syntax_error("(1))");
        expect_syntax_error("((1)");
        expect_syntax_error(")(");
        expect_syntax_error(")1(");
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
    fn parse_difference() {
        expect_value(Value::Integer(-3), "1-4");
        expect_value(Value::Integer(-3), " 1 - 4 ");
        expect_value(Value::Integer(-23), "-4-19");
        expect_value(Value::Integer(-19), "-20+1");
        expect_value(Value::Integer(-3), "0-3");
        expect_value(Value::Integer(3), "0- -3");
        expect_value(Value::Integer(-4), "-6 - -2");
        expect_value(Value::Integer(4), "+6 - +2");
        expect_value(Value::Integer(-4), "1 - 2 - 3");
        expect_value(Value::Integer(2), "1 - 2 + 3");
        expect_value(Value::Integer(0), "1 + 2 - 3");
        expect_value(Value::Integer(-6), "-1 - 2 - 3");
        expect_value(Value::Integer(4), "1 - - 3");
        expect_value(Value::Integer(-2), "1 - + 3");
        expect_value(Value::Integer(4), "1 - - - + - 3");
        expect_value(Value::Integer(-2), "1 - - - + - - 3");
        expect_syntax_error("1--3");
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
