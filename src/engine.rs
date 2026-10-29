use std::{
    fmt::{self, Debug, Display},
    ops::Add,
};

#[derive(Copy, Clone)]
pub struct Value {
    pub data: f64,
}

// Display & Debug
impl Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Value").field("data", &self.data).finish()
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Value(data={})", self.data)
    }
}

// Addition
impl<T: Into<Value>> Add<T> for Value {
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        Self {
            data: self.data + rhs.into().data,
        }
    }
}

impl From<f64> for Value {
    fn from(data: f64) -> Self {
        Self { data }
    }
}

impl Add<Value> for f64 {
    type Output = Value;
    fn add(self, rhs: Value) -> Self::Output {
        rhs + self
    }
}
