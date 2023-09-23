#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    WholeNumber(String),
    Operator(String),
    Space,
}

pub fn tokenize(expr: &str) -> Vec<Token> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_identifier() {
        assert_eq!(tokenize("x"), vec![Token::Identifier(String::from("x"))]);
        assert_eq!(tokenize("xy"), vec![Token::Identifier(String::from("xy"))]);
        assert_eq!(tokenize("z23"), vec![Token::Identifier(String::from("z23"))]);
        assert_eq!(tokenize("ffff23wwww"), vec![Token::Identifier(String::from("ffff23wwww"))]);
        assert_ne!(tokenize("_"), vec![Token::Identifier(String::from("_"))]);
        assert_ne!(tokenize("4a"), vec![Token::Identifier(String::from("4a"))]);
    }

    #[test]
    fn tokenize_whole_number() {
        assert_eq!(tokenize("1"), vec![Token::WholeNumber(String::from("1"))]);
        assert_eq!(tokenize("01234"), vec![Token::WholeNumber(String::from("01234"))]);
        assert_ne!(tokenize("1a"), vec![Token::WholeNumber(String::from("1a"))]);
        assert_ne!(tokenize("-2"), vec![Token::WholeNumber(String::from("-2"))]);
        assert_ne!(tokenize("3.14"), vec![Token::WholeNumber(String::from("3.14"))]);
    }

    #[test]
    fn tokenize_operator() {
        assert_eq!(tokenize("+"), vec![Token::Operator(String::from("+"))]);
        assert_eq!(tokenize("+-"), vec![Token::Operator(String::from("+")), Token::Operator(String::from("-"))]);
        assert_eq!(tokenize("(("), vec![Token::Operator(String::from("(")), Token::Operator(String::from("("))]);
        assert_eq!(tokenize("."), vec![Token::Operator(String::from("."))]);
    }

    #[test]
    fn tokenize_space() {
        assert_eq!(tokenize(" "), vec![Token::Space]);
        assert_eq!(tokenize("\n"), vec![Token::Space]);
        assert_eq!(tokenize("   \n\t"), vec![Token::Space]);
    }

    #[test]
    fn tokenize_multi() {
        let s = String::from;
        assert_eq!(tokenize("3.14  -x2"), vec![Token::WholeNumber(s("3")), Token::Operator(s(".")), Token::WholeNumber(s("14")), Token::Space, Token::Operator(s("-")), Token::Identifier(s("x2"))]);
    }
}
