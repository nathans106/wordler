use crate::{words, game::{self, GuessResult}};

use super::{Solver, solver::Guess};

pub struct Analysis {
    pub percent_solved: f32
}

pub fn analyse<S: Solver>(word_length: &i8, variant: &words::Variant, guesses: i8) -> Analysis {
    let words = words::with_length(word_length, variant);

    let solve_for = |word: &str| -> Vec<Guess>{
        let mut game = game::GivenWord::from(word.to_string(), *variant, guesses).unwrap();
        S::solve(&mut game)
    };

    let is_success = |solves: &Vec<Guess>| -> bool {
        match solves.last().unwrap().result {
            GuessResult::Correct => true,
            GuessResult::Incorrect(_) => false,
            GuessResult::NotInDictionary => false,
            GuessResult::GameOver(_) => false,
        }
    };

    println!("Analysing {} words", words.len());
    let successes = words.iter().map(|word| solve_for(word)).filter(is_success).count();
    Analysis { percent_solved: ((successes as f32 / words.len() as f32) * 100 as f32) }
}
