use crate::{document::Document, token::Token};

#[derive(Debug)]
// TODO: Add ability to specifiy variants
// TODO: (maybe) add ability to specifiy tokenization method
pub struct Corpus(pub Vec<Document>);

impl Corpus {
    pub fn idf(&self, term: &Token) -> f32 {
        let n = self.0.len() as f32;
        let count = self
            .0
            .iter()
            .fold(1f32, |acc, doc| f32::from(doc.0.contains_key(term)) + acc);
        (n / count).log10()
    }
}

impl FromIterator<Document> for Corpus {
    fn from_iter<T: IntoIterator<Item = Document>>(iter: T) -> Self {
        let mut corpus = Corpus(Vec::new());
        for i in iter {
            corpus.0.push(i)
        }
        corpus
    }
}
