use crate::document::{Document, Term};

#[derive(Debug)]
// TODO: Add ability to specifiy variants
// TODO: (maybe) add ability to specifiy tokenization method
pub struct Corpus<'a>(pub Vec<Document<'a>>);

impl<'a> Corpus<'a> {
    pub fn idf(&self, term: &Term) -> f32 {
        let n = self.0.len() as f32;
        let count = self
            .0
            .iter()
            .fold(1f32, |acc, doc| f32::from(doc.0.contains_key(term)) + acc);
        (n / count).log10()
    }
}

impl<'a> FromIterator<Document<'a>> for Corpus<'a> {
    fn from_iter<T: IntoIterator<Item = Document<'a>>>(iter: T) -> Self {
        let mut corpus = Corpus(Vec::new());
        for i in iter {
            corpus.0.push(i)
        }
        corpus
    }
}
