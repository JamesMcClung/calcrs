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
            Self::Sum(left, right) => Ok(left.eval()? + right.eval()?),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Self::Integer(num) => format!("{num}"),
        })
    }
}

impl std::ops::Add for Value {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Integer(lhs), Value::Integer(rhs)) => Value::Integer(lhs + rhs),
        }
    }
}
