use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use rand::{thread_rng, Rng};

#[derive(Debug, Clone)]
pub struct Corpus {
    pub words: HashSet<String>,
    pub char_probabilities: HashMap<char, f64>,
}

impl Corpus {
    pub fn from_words<T>(words: impl IntoIterator<Item = T>) -> Self
    where
        String: From<T>,
    {
        let mut char_counts: HashMap<char, usize> = HashMap::new();
        let mut total_count: usize = 0;

        let words: HashSet<String> = words
            .into_iter()
            .filter_map(|s| {
                let s = String::from(s);
                // only words with at least 3 letters and no special characters (needs improvement):
                if s.contains('\'') || s.contains('-') {
                    None
                } else if s.len() >= 3 {
                    Some(s.to_uppercase())
                } else {
                    None
                }
            })
            .collect();
        for w in words.iter() {
            for c in w.chars() {
                let entry = char_counts.entry(c).or_insert(0);
                *entry += 1;
                total_count += 1;
            }
        }

        let char_probabilities: HashMap<char, f64> = char_counts
            .into_iter()
            .map(|(k, v)| (k, v as f64 / total_count as f64))
            .collect();

        Self {
            words,
            char_probabilities,
        }
    }

    pub fn from_txt_file(path: impl AsRef<Path>) -> Result<Self, ()> {
        let content = std::fs::read_to_string(path).map_err(|_| ())?;
        let words = content.lines().filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                None
            } else {
                Some(line)
            }
        });
        Ok(Self::from_words(words))
    }

    pub fn random_char(&self) -> char {
        let v: f64 = thread_rng().gen();
        let mut acc: f64 = 0.0;
        for (c, p) in self.char_probabilities.iter() {
            acc += *p;
            if acc > v {
                return *c;
            }
        }
        panic!("should be unreachable")
    }

    pub fn line_search(&self, line: &[char]) -> Vec<(String, usize, usize)> {
        let len = line.len();
        let mut s: usize = 0;
        let mut matches: Vec<_> = vec![];
        let mut minend: usize = 0;
        while s < len - 2 {
            let mut e = len;
            'inner: while e > (s + 1).max(minend) {
                let candidate: String = line[s..e].iter().collect();
                if self.words.contains(&candidate) {
                    // add the word:
                    matches.push((candidate, s, e));
                    // set the minenc variable, such that words that are within this word are not included
                    minend = e;
                    break 'inner;
                }
                e -= 1;
            }
            s += 1;
        }
        matches
    }
}

#[cfg(test)]
mod test {
    use crate::models::corpus::Corpus;

    #[test]
    fn corpus_line_search() {
        let corpus = Corpus::from_words(["Cat", "Tomb", "Atom", "at", "Tom"]);
        // notice: at is not a part here because it is fully covered by atom:
        assert_eq!(
            corpus.line_search(&"CATOMB".chars().collect::<Vec<_>>()),
            vec![("CAT", 0, 3,), ("ATOM", 1, 5,), ("TOMB", 2, 6,),]
                .into_iter()
                .map(|(ss, s, e)| (ss.to_string(), s as usize, e as usize))
                .collect::<Vec<_>>()
        )
    }
}

// for scoring in future:
// fn main2() {
//     let corpus = Corpus::from_txt_file("assets/english3000.txt").unwrap();
//     let mut probs: Vec<_> = corpus
//         .char_probabilities
//         .clone()
//         .into_iter()
//         .map(|(c, p)| (c, p * 100.0))
//         .collect();
//     probs.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
//     dbg!(probs);
// }
