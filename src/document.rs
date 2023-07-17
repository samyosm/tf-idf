use core::fmt;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct Term<'a>(pub &'a str);

impl<'a> fmt::Display for Term<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> From<&'a str> for Term<'a> {
    fn from(value: &'a str) -> Self {
        // TODO: Validate token first
        Term(value)
    }
}

#[derive(Debug, Clone)]
pub struct Document<'a>(pub HashMap<Term<'a>, usize>);

impl<'a> Document<'a> {
    pub fn tf(&self, term: &Term) -> Option<f32> {
        if let Some(count) = self.0.get(term) {
            // TODO: Implement variants
            Some(*count as f32)
        } else {
            None
        }
    }
}

impl<'a> From<&'a str> for Document<'a> {
    fn from(value: &'a str) -> Self {
        let mut map = HashMap::new();
        for token in value.split_ascii_whitespace() {
            map.entry(token.into())
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        Document(map)
    }
}
