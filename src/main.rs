use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fmt::Display,
    fs::File,
    ops::Index,
    path::Path,
    vec,
};

use rand::{thread_rng, Rng};

fn main() {
    println!("Hello, world!");
    let corpus = Corpus::from_words(["Cat", "Tomb", "Atom", "at", "Tom"]);
    let field: &str = "ATCOMB
                       OOISIS
                       IMATOL";
    let letterfield = Letterfield::try_from(field.to_string()).unwrap();
    dbg!(&letterfield);
    let matches = letterfield.find_word_matches(&corpus);
    dbg!(matches);
    // dbg!(letterfield);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Letterfield {
    pub width: usize,
    pub height: usize,
    cols: Vec<Vec<char>>,
}

impl Letterfield {
    fn random(width: usize, height: usize, corpus: &Corpus) -> Self {
        let mut cols: Vec<Vec<char>> = vec![];
        for _ in 0..width {
            let mut col: Vec<char> = vec![];
            for _ in 0..height {
                let c = corpus.random_char();
                col.push(c);
            }
            cols.push(col);
        }

        Self {
            width,
            height,
            cols,
        }
    }

    fn find_word_matches_and_replace(
        &mut self,
        corpus: &Corpus,
    ) -> (Vec<WordMatch>, HashSet<(usize, usize)>) {
        todo!()
    }

    /// matches should be non overlapping, so if the letterfield is:
    /// Y O U T U B E   
    /// O O O O O O O
    /// then only YOUTUBE and TO are matches, not TUBE YOU or BE, because they are already contained in a match.s
    fn find_word_matches(&self, corpus: &Corpus) -> Vec<WordMatch> {
        // for horizontal matches: check each line:
        //      start from left with start: 0 with end: width
        //      if hit, can return for that line
        //      if not hit, decrement end, until end == start+2 (no single letter words allowed)
        //          if still no hit, increment start by one. (until start is width - 2)

        let mut matches: Vec<WordMatch> = vec![];

        // check all cols:
        for (col_index, col) in self.cols.iter().enumerate() {
            for (word, s, e) in corpus.line_search(&col[..]) {
                let positions: Vec<(char, usize, usize)> =
                    (s..e).map(|i| (col[i], i, col_index)).collect();
                let word_match = WordMatch {
                    word,
                    positions,
                    kind: WordMatchKind::Column,
                };
                matches.push(word_match);
            }
        }
        // check all rows:
        for row_index in 0..self.height {
            let row: Vec<char> = self.cols.iter().map(|col| col[row_index]).collect();
            for (word, s, e) in corpus.line_search(&row[..]) {
                let positions: Vec<(char, usize, usize)> =
                    (s..e).map(|i| (row[i], row_index, i)).collect();
                let word_match = WordMatch {
                    word,
                    positions,
                    kind: WordMatchKind::Row,
                };
                matches.push(word_match);
            }
        }

        // check lower diagonals
        // lets say width 4, height 9: i = 3 =>
        let mut diag_start_points: Vec<(usize, usize)> = vec![];
        for i in 0..(self.height - 2) {
            diag_start_points.push((0usize, i));
        }
        for i in 1..(self.width - 2) {
            diag_start_points.push((i, 0usize));
        }
        let diags_and_start_points: Vec<(Vec<char>, (usize, usize))> = diag_start_points
            .into_iter()
            .map(|start_point| {
                let (mut x, mut y) = start_point;
                let travel = (self.width - x).min(self.height - y);
                let mut diag: Vec<char> = vec![];

                for _ in 0..travel {
                    let c = self.cols[x][y];
                    diag.push(c);
                    x += 1;
                    y += 1;
                }
                (diag, start_point)
            })
            .collect();

        for (diag, start_point) in diags_and_start_points {
            for (word, s, e) in corpus.line_search(&diag[..]) {
                let positions: Vec<(char, usize, usize)> = (s..e)
                    .map(|i| (diag[i], start_point.0 + i, start_point.1 + i))
                    .collect();
                let word_match = WordMatch {
                    word,
                    positions,
                    kind: WordMatchKind::Diagonal,
                };
                matches.push(word_match);
            }
        }
        matches
    }
}

#[derive(Debug, Clone)]
pub struct WordMatch {
    word: String,
    // char, x as in -->, y as in |
    //                            V
    positions: Vec<(char, usize, usize)>,
    kind: WordMatchKind,
}

#[derive(Debug, Clone)]
enum WordMatchKind {
    Column,
    Row,
    Diagonal,
}

impl TryFrom<String> for Letterfield {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let lines: Vec<Vec<char>> = value
            .lines()
            .map(|line| {
                line.replace(" ", "")
                    .to_uppercase()
                    .trim()
                    .chars()
                    .collect::<Vec<char>>()
            })
            .collect();
        let cols = transpose_vecs(&lines);
        let width = cols.len();
        let height = cols[0].len();
        Ok(Letterfield {
            cols,
            width,
            height,
        })
    }
}

impl Display for Letterfield {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rows = transpose_vecs(&self.cols);
        let output = rows
            .into_iter()
            .map(|line| {
                line.into_iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{output}")
    }
}

#[derive(Debug, Clone)]
pub struct Corpus {
    words: HashSet<String>,
    char_probabilities: HashMap<char, f64>,
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
            .map(|s| String::from(s).to_uppercase())
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

    fn random_char(&self) -> char {
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

    fn line_search(&self, line: &[char]) -> Vec<(String, usize, usize)> {
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

fn transpose_vecs<T: Clone>(vecs: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let inner_len = vecs.len();
    let outer_len = vecs[0].len();

    let mut outer: Vec<Vec<T>> = vec![];
    for o in 0..outer_len {
        let mut inner: Vec<T> = vec![];
        for i in 0..inner_len {
            inner.push(vecs[i][o].clone());
        }
        outer.push(inner);
    }
    outer
}

#[cfg(test)]
mod test {
    use crate::{Corpus, Letterfield};

    #[test]
    fn letterfield_to_and_from_str() {
        let corpus = Corpus::from_txt_file("./assets/words.txt").unwrap();
        let letterfield = Letterfield::random(4, 5, &corpus);
        let letterfield2: Letterfield = letterfield.to_string().try_into().unwrap();
        assert_eq!(letterfield, letterfield2);
    }

    #[test]
    fn corpus_line_search() {
        let corpus = Corpus::from_words(["Cat", "Tomb", "Atom", "at", "Tom"]);
        assert_eq!(
            corpus.line_search(&"CATOMB".chars().collect::<Vec<_>>()),
            vec![("CAT", 0, 3,), ("ATOM", 1, 5,), ("TOMB", 2, 6,),]
                .into_iter()
                .map(|(ss, s, e)| (ss.to_string(), s as usize, e as usize))
                .collect::<Vec<_>>()
        )
    }
}

// x, y
#[derive(Debug, Clone, Copy)]
pub struct Int2(pub usize, pub usize);

#[derive(Debug, Clone, Copy)]
struct Int2i(pub isize, pub isize);

#[derive(Debug, Clone)]
struct Field<T> {
    pub width: usize,
    pub height: usize,
    cols: Vec<Vec<T>>,
}

impl<T: Clone> Field<T> {
    pub fn transpose(&self) -> Field<T> {
        Field {
            width: self.height,
            height: self.width,
            cols: transpose_vecs(&self.cols),
        }
    }
}

impl<T> Index<Int2> for Field<T> {
    type Output = T;

    fn index(&self, index: Int2) -> &Self::Output {
        &self.cols[index.0][index.1]
    }
}
