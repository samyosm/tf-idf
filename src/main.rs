use document::Document;

use crate::corpus::Corpus;

mod corpus;
mod document;

fn main() {
    let corpus: Corpus = vec![
        "Hello, world!",
        "test test test nice nice lol",
        "How are you guys doing?",
        "This test is a quote!",
    ]
    .iter()
    .map(|&s| Document::from(s))
    .collect();

    for d in &corpus.0 {
        for term in d.0.keys() {
            if let Some(tf) = d.tf(term) {
                println!("{}: tf({}) idf({})", term, tf, corpus.idf(term));
            }
        }
    }
}
