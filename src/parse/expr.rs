use std::fmt::Display;

use super::Error;

#[derive(Debug)]
pub enum Expression {
    Constant(Value),
    Sum(Box<Expression>, Box<Expression>),
}

#[derive(Debug)]
pub enum Value {
    Integer(i64),
}

impl Expression {
    pub fn eval(self) -> Result<Value, Error> {
        match self {
            Self::Constant(c) => Ok(c),
            Self::Sum(left, right) => match (left.eval()?, right.eval()?) {
                (Value::Integer(left), Value::Integer(right)) => Ok(Value::Integer(left + right)),
            },
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(num) => f.write_str(&format!("{num}"))?,
        };
        Ok(())
    }
}
