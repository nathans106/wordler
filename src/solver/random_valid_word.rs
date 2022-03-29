use rand::prelude::IteratorRandom;

use super::Solver;
use super::solver::Guess;
use super::word_filter::WordFilter;
use crate::game::{Game, GuessResult};
use crate::words;

pub struct RandomValidWord;

impl Solver for RandomValidWord {
    fn solve(game: &mut dyn Game) -> Vec<Guess> {
        let words = words::with_length(game.word_length(), game.words_variant());
        let mut filter = WordFilter::from(*game.word_length());
        let mut rng = rand::thread_rng();
        let mut guesses = Vec::new();

        loop {
            let guess = words.iter().filter(|word| filter.call(word)).choose(&mut rng).unwrap();
            let result = game.guess(guess);
            guesses.push(Guess{result: result, word: guess.to_string() });

            match game.guess(guess) {
                GuessResult::Correct => return guesses,
                GuessResult::Incorrect(statuses) => filter.add(guess, &statuses),
                GuessResult::NotInDictionary => unimplemented!(),
                GuessResult::GameOver(_) => return guesses,
            }
        }
    }
}