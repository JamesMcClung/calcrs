use std::fmt::Display;

#[derive(Debug)]
pub enum Expression {
    Constant(Value),
}

#[derive(Debug)]
pub enum Value {
    Integer(i64),
}

impl Expression {
    pub fn eval(self) -> Result<Value, String> {
        match self {
            Self::Constant(c) => Ok(c),
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
