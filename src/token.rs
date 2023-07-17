use std::fmt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct Token(pub String);

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for Token {
    fn from(value: String) -> Self {
        // TODO: Validate token first
        Token(value)
    }
}
