use std::collections::HashMap;

use crate::{token::Token, tokenizer::tokenize};

#[derive(Debug, Clone)]
pub struct Document(pub HashMap<Token, usize>);

impl Document {
    pub fn tf(&self, term: &Token) -> Option<f32> {
        if let Some(count) = self.0.get(term) {
            // TODO: Implement variants
            Some(*count as f32)
        } else {
            None
        }
    }
}

impl<'a> From<&'a str> for Document {
    fn from(value: &'a str) -> Self {
        let mut map = HashMap::new();
        for token in tokenize(value) {
            map.entry(Token(token))
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        Document(map)
    }
}
