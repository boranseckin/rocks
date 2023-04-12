use std::fmt;

/// Represents a literal value in the language.
#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    String(String),
    Number(f32),
    Bool(bool),
    Null,
}

impl Literal {
    /// Returns the literal as a number.
    pub fn as_number(&self) -> f32 {
        match self {
            Literal::Number(n) => *n,
            Literal::Null => 0.0,
            Literal::Bool(b) => if *b { 1.0 } else { 0.0 },
            Literal::String(s) => s.parse::<f32>().unwrap_or(0.0),
        }
    }

    /// Returns the literal as a boolean.
    pub fn as_bool(&self) -> bool {
        match self {
            Literal::Bool(b) => *b,
            Literal::Null => false,
            _ => true,
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

impl Eq for Literal {}
