// use std::str::SplitWhitespace;

use std::collections::{HashMap, HashSet};

const WORDS: &str = include_str!("words_alpha.txt");

fn main() {
    let mut words_list: Vec<WordSet> = Vec::new();
    for word in WORDS
        .split_whitespace()
        .filter(|w| w.len() == 5)
        .map(|s| s.to_uppercase())
    {
        let wset = WordSet::from_word(&word);

        // println!("{}:    {word}", WordSet::from_word(&word).show());

        // ensure only words with 5 unique letters are counted
        if wset.size() != 5 {
            continue;
        }
        words_list.push(wset);
    }

    words_list.sort(); // due to the representation, this'll be 'backwards', but it doesn't matter
    words_list.dedup();

    println!("created word list");

    // words_hmap[w] is all the words that
    // - are disjoint from w
    // appear later in words_list.
    let mut words_hmap: HashMap<WordSet, HashSet<WordSet>> = HashMap::new();
    for (ix, word) in (&words_list).iter().enumerate() {
        let mut s: HashSet<WordSet> = HashSet::new();

        for word2 in &words_list[ix + 1..] {
            if word.intersection(*word2) == WordSet::empty() {
                s.insert(*word2);
            }
        }

        words_hmap.insert(*word, s);
    }

    println!("initialized hashmap");

    // println!("{:?}", &words_list[0..5]);

    // for word in &words_hmap[&WordSet::from_word("units")] {
    // println!("{}", &word.show());
    // }

    // find the solutions!
    let mut solutions: Vec<Solution> = Vec::new();
    for (word, others) in words_hmap.iter() {
        for word2 in others {
            for word3 in &words_hmap[word2] {
                if word.intersection(*word3) != WordSet::empty() {
                    continue;
                }

                for word4 in &words_hmap[word3] {
                    if word.intersection(*word4) != WordSet::empty()
                        || word2.intersection(*word4) != WordSet::empty()
                    {
                        continue;
                    }

                    for word5 in &words_hmap[word4] {
                        if word.intersection(*word5) != WordSet::empty()
                            || word2.intersection(*word5) != WordSet::empty()
                            || word3.intersection(*word5) != WordSet::empty()
                        {
                            continue;
                        }
                        // we've found our 5-tuple!!!
                        solutions.push(Solution(*word, *word2, *word3, *word4, *word5));
                    }
                }
            }
        }
    }

    println!("found all solutions");

    let mut inv: HashMap<WordSet, String> = HashMap::new();

    for (idx, solution) in solutions.iter().enumerate() {
        println!("solution #{}:", idx + 1);

        let Solution(w1, w2, w3, w4, w5) = *solution;

        for w_n in &[w1, w2, w3, w4, w5] {
            if let Some(w) = inv.get(w_n) {
                println!("{}", w);
            } else {
                // find the first word that gives w_n and save it in inv to save time if it shows up again
                for word in WORDS.split_whitespace().filter(|wd| wd.len() == 5) {
                    if WordSet::from_word(word) == *w_n {
                        println!("{}", word);
                        inv.insert(*w_n, word.to_owned());
                    }
                }
            }
        }

        println!();
    }
}

/// represents the set of letters of a word as a bit field
/// the first six bits are unused, and then it goes A-Z, with a 1 if it's included in the word.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct WordSet(u32);

impl WordSet {
    fn show(self) -> String {
        let mut res = String::new();
        for i in 0..26 {
            if self.0 & (1 << (25 - i)) > 0 {
                res.push((b'A' + i) as char);
            } else {
                res.push('-');
            }
        }
        res
    }

    fn from_word(word: &str) -> Self {
        let mut res = 0;
        for ix in word.to_uppercase().as_bytes().iter().map(|c| c - b'A') {
            res |= 1 << (25 - ix);
        }
        Self(res)
    }

    fn empty() -> Self {
        Self(0)
    }
    fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
    fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
    fn size(self) -> u32 {
        self.0.count_ones()
    }
}

#[derive(Debug, Copy, Clone)]
struct Solution(WordSet, WordSet, WordSet, WordSet, WordSet);
