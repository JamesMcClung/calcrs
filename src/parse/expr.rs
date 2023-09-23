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
