use rayon::prelude::*;

use std::{
    collections::{HashMap, HashSet},
    sync::{Mutex, RwLock},
};

const WORDS: &[u8] = include_bytes!("words_alpha.txt");

fn main() {
    let mut words_list: Vec<WordSet> = WORDS
        .par_split(u8::is_ascii_whitespace)
        .filter(|w| w.len() == 5)
        .map(WordSet::from_word)
        .filter(|w| w.size() == 5)
        .collect();

    words_list.par_sort(); // due to the representation, this'll be 'backwards', but it doesn't matter
    words_list.dedup();

    println!("created word list");

    // words_hmap[w] is all the words that
    // are disjoint from w and appear later in words_list.
    let words_hmap: Mutex<HashMap<WordSet, HashSet<WordSet>>> = Mutex::new(HashMap::new());
    words_list.iter().enumerate().for_each(|(ix, word)| {
        let mut s: HashSet<WordSet> = HashSet::new();

        for word2 in &words_list[ix + 1..] {
            if word.intersection(*word2) == WordSet::empty() {
                s.insert(*word2);
            }
        }

        words_hmap.lock().unwrap().insert(*word, s);
    });

    let words_hmap = words_hmap.into_inner().unwrap();
    println!("initialized hashmap");

    // find the solutions!
    let solutions: Mutex<Vec<Solution>> = Mutex::new(Vec::new());
    words_hmap.par_iter().for_each(|(word, others)| {
        others.par_iter().for_each(|word2| {
            words_hmap[word2].par_iter().for_each(|word3| {
                if word.intersection(*word3) != WordSet::empty() {
                    return;
                }

                words_hmap[word3].par_iter().for_each(|word4| {
                    if word.intersection(*word4) != WordSet::empty()
                        || word2.intersection(*word4) != WordSet::empty()
                    {
                        return;
                    }

                    words_hmap[word4].par_iter().for_each(|word5| {
                        if word.intersection(*word5) != WordSet::empty()
                            || word2.intersection(*word5) != WordSet::empty()
                            || word3.intersection(*word5) != WordSet::empty()
                        {
                            return;
                        }
                        // we've found our 5-tuple!!!
                        let mut slns = solutions.lock().unwrap();
                        slns.push(Solution(*word, *word2, *word3, *word4, *word5));
                    })
                })
            })
        })
    });

    let solutions = solutions.into_inner().unwrap();

    println!("found all solutions");

    let inv: RwLock<HashMap<WordSet, Vec<u8>>> = RwLock::new(HashMap::new());
    solutions.par_iter().for_each(|solution| {
        let Solution(w1, w2, w3, w4, w5) = *solution;

        let res = Mutex::new(b"solution:\n".to_vec());

        for w_n in &[w1, w2, w3, w4, w5] {
            let inv_guard = inv.read().unwrap();
            if let Some(word) = inv_guard.get(w_n) {
                let mut res = res.lock().unwrap();
                res.extend_from_slice(word);
                res.push(b'\n');
                drop(inv_guard)
            } else {
                drop(inv_guard);
                // find the first word that gives w_n and save it in inv to save time if it shows up again
                let found_word = WORDS
                    .par_split(u8::is_ascii_whitespace)
                    .filter(|wd| wd.len() == 5)
                    .find_any(|word| WordSet::from_word(word) == *w_n)
                    .unwrap();

                let mut res = res.lock().unwrap();
                res.extend_from_slice(found_word);
                res.push(b'\n');
                drop(res);

                inv.write().unwrap().insert(*w_n, found_word.to_owned());
            }
        }

        let res = res.into_inner().unwrap();
	unsafe {
	    // SAFETY: all words should consist only of valid ascii
            println!("{}", &std::str::from_utf8_unchecked(&res));
	}
    });

    println!("done! found {} solutions", solutions.len());
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

    fn from_word(word: &[u8]) -> Self {
        let mut res = 0;
        for ix in word.iter().map(|c| c.to_ascii_uppercase() - b'A') {
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
