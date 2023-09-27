use super::Error;

#[derive(Debug)]
pub enum Expression {
    Constant(Value),
    Sum(Box<Expression>, Box<Expression>),
    UnaryPlus(Box<Expression>),
    UnaryMinus(Box<Expression>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Value {
    Integer(i64),
}

impl Expression {
    pub fn eval(self) -> Result<Value, Error> {
        match self {
            Self::Constant(c) => Ok(c),
            Self::Sum(left, right) => Ok(left.eval()? + right.eval()?),
            Self::UnaryPlus(expr) => Ok(expr.eval()?),
            Self::UnaryMinus(expr) => Ok(-expr.eval()?),
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

impl std::ops::Neg for Value {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Value::Integer(val) => Value::Integer(-val),
        }
    }
}
