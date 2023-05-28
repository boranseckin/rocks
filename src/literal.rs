use std::fmt;

/// Represents a literal value in the language.
/// This is used to represent strings, numbers, booleans, and null.
#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    String(String),
    Number(f32),
    Bool(bool),
    Null,
}

impl Literal {
    /// Returns the literal value as a number.
    /// - If the literal is a number, it will return the number.
    /// - If the literal is a boolean, it will return 1.0 if true, 0.0 if false.
    /// - If the literal is null, it will return 0.0.
    /// - If the literal is a string, it will attempt to parse it as a number or return 0.0.
    pub fn as_number(&self) -> f32 {
        match self {
            Literal::Number(n) => *n,
            Literal::Bool(b) => if *b { 1.0 } else { 0.0 },
            Literal::Null => 0.0,
            Literal::String(s) => s.parse::<f32>().unwrap_or(0.0),
        }
    }

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

impl From<f32> for Literal {
    fn from(n: f32) -> Self {
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

// This should not be used, but is required for Hashing
impl Eq for Literal {}
