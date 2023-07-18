use std::{
    collections::HashSet,
    fmt::{format, Display},
};

use super::{
    array2d::{Array2D, Array2DIter, Int2},
    corpus::Corpus,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Letterfield {
    id_count: u32,
    field: Array2D<(u32, char)>,
}

impl Letterfield {
    /// letter can either be moved horizontally or vertically.
    ///
    /// returns the ids of the tiles that were effected.
    ///
    /// # panics
    ///
    /// if the porvided positions out of bounds or not in same row/column
    pub fn move_letter(&mut self, old_pos: Int2, new_pos: Int2) {
        assert!(old_pos.x == new_pos.x || old_pos.y == new_pos.y);

        assert!(
            old_pos.x < self.width()
                && new_pos.x < self.width()
                && old_pos.y < self.height()
                && new_pos.y < self.height()
        );
        if old_pos == new_pos {
            return;
        }

        let element = self.field[old_pos];
        if new_pos.x < old_pos.x {
            // horizontal move to the left
            assert_eq!(new_pos.y, old_pos.y);
            let y = new_pos.y;
            for x in (new_pos.x..old_pos.x).rev() {
                self.field[Int2 { x: x + 1, y }] = self.field[Int2 { x, y }]
            }
        } else if new_pos.x > old_pos.x {
            // horizontal move to the right
            assert_eq!(new_pos.y, old_pos.y);
            let y = new_pos.y;
            for x in (old_pos.x + 1)..=new_pos.x {
                self.field[Int2 { x: x - 1, y }] = self.field[Int2 { x, y }]
            }
        } else if new_pos.y < old_pos.y {
            // vertical move up
            assert_eq!(new_pos.x, old_pos.x);
            let x = new_pos.x;
            for y in (new_pos.y..old_pos.y).rev() {
                self.field[Int2 { x, y: y + 1 }] = self.field[Int2 { x, y }]
            }
        } else if new_pos.y > old_pos.y {
            // vertical move down
            assert_eq!(new_pos.x, old_pos.x);
            let x = new_pos.x;

            for y in (old_pos.y + 1)..=new_pos.y {
                self.field[Int2 { x, y: y - 1 }] = self.field[Int2 { x, y }]
            }
        }
        self.field[new_pos] = element;
    }

    pub fn new(field: Array2D<(u32, char)>) -> Self {
        Self { id_count: 0, field }
    }

    pub fn width(&self) -> usize {
        self.field.width
    }

    pub fn height(&self) -> usize {
        self.field.height
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.field.width, self.field.height)
    }

    fn next_id(&mut self) -> u32 {
        self.id_count += 1;
        self.id_count
    }

    pub fn iter<'a>(&'a self) -> Array2DIter<'a, (u32, char)> {
        self.field.iter()
    }

    pub fn random_with_no_matches(width: usize, height: usize, corpus: &Corpus) -> Self {
        let mut letterfield = Letterfield::random(width, height, corpus);
        // let mut c = 0;
        loop {
            let (matches, _replacements) =
                letterfield.find_word_matches_and_fill_spaces_randomly(corpus);
            // c += 1;
            // dbg!(c, matches.len());
            if matches.is_empty() {
                break letterfield;
            }
        }
    }

    pub fn random(width: usize, height: usize, corpus: &Corpus) -> Self {
        let mut id_count = 0;
        let mut cols: Vec<Vec<(u32, char)>> = vec![];
        for _ in 0..width {
            let mut col: Vec<(u32, char)> = vec![];
            for _ in 0..height {
                let c = corpus.random_char();
                col.push((id_count, c));
                id_count += 1;
            }
            cols.push(col);
        }

        let field = Array2D::try_from(cols).unwrap();

        Self { id_count, field }
    }

    pub fn find_word_matches_and_fill_spaces_randomly(
        &mut self,
        corpus: &Corpus,
    ) -> (Vec<WordMatch>, HashSet<(Int2, (u32, char))>) {
        let word_matches = self.find_word_matches(corpus);
        let positions: HashSet<Int2> = word_matches
            .iter()
            .flat_map(|m| m.tiles.iter().map(|(_, _, pos)| *pos))
            .collect();
        let replacements: HashSet<(Int2, (u32, char))> = positions
            .into_iter()
            .map(|pos| {
                let c = corpus.random_char();
                let id = self.next_id();
                self.field[pos] = (id, c);
                (pos, (id, c))
            })
            .collect();
        (word_matches, replacements)
    }

    // pub fn chars_and_positions(&self) -> Vec<(char, Int2)> {
    //     let mut res: Vec<(char, Int2)> = vec![];
    //     for x in 0..self.width() {
    //         for y in 0..self.height() {
    //             let pos = Int2 { x, y };
    //             let c = self.field[pos];
    //             res.push((c, pos));
    //         }
    //     }
    //     res
    // }

    /// matches should be non overlapping, so if the letterfield is:
    /// Y O U T U B E   
    /// O O O O O O O
    /// then only YOUTUBE and TO are matches, not TUBE YOU or BE, because they are already contained in a match.s
    pub fn find_word_matches(&self, corpus: &Corpus) -> Vec<WordMatch> {
        // for horizontal matches: check each line:
        //      start from left with start: 0 with end: width
        //      if hit, can return for that line
        //      if not hit, decrement end, until end == start+2 (no single letter words allowed)
        //          if still no hit, increment start by one. (until start is width - 2)

        let mut word_matches: Vec<WordMatch> = vec![];

        // check all cols:
        for (line, start) in self.field.cols_2() {
            for (word, s, e) in corpus.line_search(&line[..]) {
                let tiles = (s..e)
                    .map(|i| {
                        let pos = start.with_y(i);
                        let (id, ch) = self.field[pos];
                        (id, ch, pos)
                    })
                    .collect();
                let word_match = WordMatch {
                    word,
                    tiles,
                    kind: WordMatchKind::Column,
                };
                word_matches.push(word_match);
            }
        }
        // check all rows:
        for (line, start) in self.field.rows_2() {
            for (word, s, e) in corpus.line_search(&line[..]) {
                let tiles = (s..e)
                    .map(|i| {
                        let pos = start.with_x(i);
                        let (id, ch) = self.field[pos];
                        (id, ch, pos)
                    })
                    .collect();
                let word_match = WordMatch {
                    word,
                    tiles,
                    kind: WordMatchKind::Row,
                };
                word_matches.push(word_match);
            }
        }
        // check top-left to bottom-right diags:
        for (line, start) in self.field.diags_2(3) {
            for (word, s, e) in corpus.line_search(&line[..]) {
                let tiles = (s..e)
                    .map(|i| {
                        let pos = Int2 {
                            x: start.x + i,
                            y: start.y + i,
                        };
                        let (id, ch) = self.field[pos];
                        (id, ch, pos)
                    })
                    .collect();
                let word_match = WordMatch {
                    word,
                    tiles,
                    kind: WordMatchKind::Row,
                };
                word_matches.push(word_match);
            }
        }

        // return all matches found:
        word_matches
    }

    pub fn to_detail_string(&self) -> String {
        let rows = self.field.rows();
        let output = rows
            .into_iter()
            .map(|(line, _)| {
                line.into_iter()
                    .map(|(id, c)| format!("({id}, {c})"))
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join("\n");

        output
    }
}

#[derive(Debug, Clone)]
pub struct WordMatch {
    pub word: String,
    // char, x as in -->, y as in |
    //                            V
    pub tiles: Vec<(u32, char, Int2)>,
    pub kind: WordMatchKind,
}

#[derive(Debug, Clone)]
pub enum WordMatchKind {
    Column,
    Row,
    Diagonal,
}

impl TryFrom<String> for Letterfield {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut id = 0;
        let lines: Vec<Vec<(u32, char)>> = value
            .lines()
            .map(|line| {
                line.replace(' ', "")
                    .to_uppercase()
                    .trim()
                    .chars()
                    .map(|c| {
                        id += 1;
                        (id, c)
                    })
                    .collect::<Vec<(u32, char)>>()
            })
            .collect();
        let field = Array2D::try_from(lines)?.transpose();

        Ok(Letterfield::new(field))
    }
}

impl Display for Letterfield {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rows = self.field.rows();
        let output = rows
            .into_iter()
            .map(|(line, _)| {
                line.into_iter()
                    .map(|(_, c)| c.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{output}")
    }
}

#[cfg(test)]
mod test {
    use crate::models::{corpus::Corpus, letterfield::Letterfield};

    #[test]
    fn letterfield_to_and_from_str() {
        let corpus = Corpus::from_words(["Cat", "Tomb", "Atom", "at", "Tom"]);
        let letterfield = Letterfield::random(4, 5, &corpus);
        let letterfield2: Letterfield = letterfield.to_string().try_into().unwrap();
        assert_eq!(letterfield, letterfield2);
    }

    #[test]
    fn letterfield_matches() {
        let corpus = Corpus::from_txt_file("assets/english3000.txt").unwrap();
        let letterfield = Letterfield::random(20, 20, &corpus);
        // probabilistic test but should be fine: (on average 30 matches in 20x20)
        assert!(!letterfield.find_word_matches(&corpus).is_empty());
        let letterfield = Letterfield::random_with_no_matches(20, 20, &corpus);
        assert!(letterfield.find_word_matches(&corpus).is_empty());
    }
}
