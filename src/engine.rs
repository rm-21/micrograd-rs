use std::fmt::Debug;

pub struct Value {
    pub data: f64,
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Value").field("data", &self.data).finish()
    }
}
