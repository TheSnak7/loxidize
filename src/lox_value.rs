use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum LoxValue {
    Number(f64),
}

impl From<f64> for LoxValue {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl From<i32> for LoxValue {
    fn from(value: i32) -> Self {
        Self::Number(f64::from(value))
    }
}

impl Default for LoxValue {
    fn default() -> Self {
        // FIXME: replace with nil later
        LoxValue::Number(0.0)
    }
}

impl Display for LoxValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoxValue::Number(num) => write!(f, "{num}"),
        }
    }
}
