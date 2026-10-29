use std::{
    fmt::{self, Debug, Display},
    ops::{Add, Div, Mul, Neg, Sub},
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

impl From<f64> for Value {
    fn from(data: f64) -> Self {
        Self { data }
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

impl Add<Value> for f64 {
    type Output = Value;
    fn add(self, rhs: Value) -> Self::Output {
        rhs + self
    }
}

// Multiplication
impl<T: Into<Value>> Mul<T> for Value {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            data: self.data * rhs.into().data,
        }
    }
}

impl Mul<Value> for f64 {
    type Output = Value;
    fn mul(self, rhs: Value) -> Self::Output {
        rhs * self
    }
}

// Negation & Subtraction
impl Neg for Value {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl<T: Into<Value>> Sub<T> for Value {
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        self + (-rhs.into())
    }
}

impl Sub<Value> for f64 {
    type Output = Value;
    fn sub(self, rhs: Value) -> Self::Output {
        self + (-rhs)
    }
}

// Pow & Division
impl Value {
    pub fn pow(self, n: f64) -> Self {
        Self {
            data: self.data.powf(n),
        }
    }
}

impl<T: Into<Value>> Div<T> for Value {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        self * rhs.into().pow(-1.0)
    }
}

impl Div<Value> for f64 {
    type Output = Value;
    fn div(self, rhs: Value) -> Self::Output {
        self * rhs.pow(-1.0)
    }
}
