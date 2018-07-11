use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

pub struct Searcher {
    search_term: String,
}

impl Searcher {

    pub fn new(search_term: String) -> Self {
        Self {
            search_term: search_term.clone(),
        }
    }

    pub fn search(&self) -> Vec<String> {
        let word_list = lines_from_file("words_alpha.txt");
        let mut candidates = Vec::new();
        for word in &word_list {
            if word.contains(self.search_term.as_str()) {
                candidates.push(word.to_string());
            }
        }
        candidates
    }

}

fn lines_from_file<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
