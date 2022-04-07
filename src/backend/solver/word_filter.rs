use std::{collections::{HashSet, HashMap}};

use crate::game::{LetterStatuses, LetterStatus};


pub struct WordFilter {
    word_length: i8,
    not_present: HashSet<char>,
    disallowed: HashMap<usize, HashSet<char>>,
    unknown_pos: HashSet<char>,
    known: HashMap<usize, char>
}

impl WordFilter {
    pub fn from(word_length: i8) -> WordFilter{
        WordFilter { word_length: word_length, not_present: HashSet::new(), disallowed: HashMap::new(), unknown_pos: HashSet::new(), known: HashMap::new() }
    }

    pub fn add(&mut self, word: &str, statuses: &LetterStatuses) {
        for (i, (c, status)) in word.chars().zip(statuses.iter()).enumerate() {
            match status {
                LetterStatus::Correct => {
                    self.unknown_pos.remove(&c);
                    self.known.insert(i, c);
                },
                LetterStatus::WrongPosition => {
                    if !self.disallowed.contains_key(&i) {
                        self.disallowed.insert(i, HashSet::new());
                    }

                    self.disallowed.get_mut(&i).unwrap().insert(c);
                },
                LetterStatus::NotPresent => drop(self.not_present.insert(c))
            }
        }
    }

    // TODO - impl Fn trait once feature is in Rust stable
    pub fn call(&self, word: &str) -> bool {
        assert!(word.len() == self.word_length as usize);

        if self.known.iter().any(|(i, c)| word.chars().nth(*i as usize).unwrap() != *c) {
            return false;
        }

        let mut need_to_find = self.unknown_pos.clone();
        for (i, c) in word.chars().enumerate() {
            let known = self.known.get(&i);
            if known.is_some() && known.unwrap() != &c {
                return false;
            }

            if self.not_present.contains(&c) {
                return false;
            }

            let disallowed = self.disallowed.get(&i);
            if disallowed.is_some() && disallowed.unwrap().contains(&c) {
                return false;
            }

            need_to_find.remove(&c);
        }

        if need_to_find.len() > 0 {
            return false;
        }

        return true;
    }
}
