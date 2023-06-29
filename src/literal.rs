use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Not, Neg};

/// Represents a literal value in the language.
/// This is used to represent strings, numbers, booleans, and null.
#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

impl Literal {
    /// Returns the literal value as a boolean.
    /// - If the literal is a boolean, it will return the boolean.
    /// - If the literal is a number, it will return true if the number is not 0.0.
    /// - If the literal is null, it will return false.
    /// - If the literal is a string, it will return true if the string is not empty.
    pub fn as_bool(&self) -> bool {
        match self {
            Literal::Bool(b) => *b,
            Literal::Number(b) => *b != 0.0,
            Literal::Null => false,
            Literal::String(b) => !b.is_empty(),
        }
    }

    pub fn type_str(&self) -> &str {
        match self {
            Literal::String(_) => "string",
            Literal::Number(_) => "number",
            Literal::Bool(_) => "boolean",
            Literal::Null => "null",
        }
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::String(left), Self::String(right)) => left == right,
            (Self::Number(left), Self::Number(right)) => left == right,
            (Self::Bool(left), Self::Bool(right)) => left == right,
            (Self::Null, Self::Null) => true,
            _ => false,
        }
    }
}

impl PartialOrd for Literal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Number(left), Self::Number(right)) => left.partial_cmp(right),
            _ => None,
        }
    }
}

// This should not be used, but is required for Hashing
impl Eq for Literal {}

impl Add for Literal {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(left), Self::Number(right)) => Some(Self::Number(left + right)),
            (Self::String(left), Self::String(right)) => Some(Self::String(left + &right)),
            _ => None,
        }
    }
}

impl Sub for Literal {
    type Output = Option<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(left), Self::Number(right)) => Some(Self::Number(left - right)),
            _ => None,
        }
    }
}

impl Mul for Literal {
    type Output = Option<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(left), Self::Number(right)) => Some(Self::Number(left * right)),
            _ => None,
        }
    }
}

impl Div for Literal {
    type Output = Option<Self>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(left), Self::Number(right)) => Some(Literal::Number(left / right)),
            _ => None,
        }
    }
}

impl Not for Literal {
    type Output = Option<Self>;

    fn not(self) -> Self::Output {
        match self {
            Self::Bool(right) => Some(Self::Bool(!right)),
            Self::Null => Some(Self::Bool(true)),
            _ => None,
        }
    }
}

impl Neg for Literal {
    type Output = Option<Self>;

    fn neg(self) -> Self::Output {
        match self {
            Self::Number(right) => Some(Self::Number(-right)),
            _ => None,
        }
    }
}

impl From<&str> for Literal {
    fn from(s: &str) -> Self {
        Literal::String(String::from(s))
    }
}

impl From<String> for Literal {
    fn from(s: String) -> Self {
        Literal::String(s)
    }
}

impl From<f64> for Literal {
    fn from(n: f64) -> Self {
        Literal::Number(n)
    }
}

impl From<bool> for Literal {
    fn from(b: bool) -> Self {
        Literal::Bool(b)
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::String(s) => write!(f, "{s}"),
            Literal::Number(n) => write!(f, "{n}"),
            Literal::Bool(b) => write!(f, "{b}"),
            Literal::Null => write!(f, "null"),
        }
    }
}
