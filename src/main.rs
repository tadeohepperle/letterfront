use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    ops::Index,
    path::Path,
    vec,
};

use models::corpus::Corpus;
use rand::{thread_rng, Rng};

pub mod models;

fn main() {
    let corpus = Corpus::from_txt_file("assets/english3000.txt").unwrap();
    let mut probs: Vec<_> = corpus
        .char_probabilities
        .clone()
        .into_iter()
        .map(|(c, p)| (c, p * 100.0))
        .collect();
    probs.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    dbg!(probs);
}

fn main2() {
    let corpus = Corpus::from_txt_file("assets/english3000.txt").unwrap();
    let mut probs: Vec<_> = corpus
        .char_probabilities
        .clone()
        .into_iter()
        .map(|(c, p)| (c, p * 100.0))
        .collect();
    probs.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    dbg!(probs);
}
