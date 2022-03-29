use crate::{words, game::{self, GuessResult}};

use super::{Solver, solver::Guess};

pub struct Analysis {
    pub percent_solved: f32,
    pub mean: f32,
    pub median: i8,
    pub min: i8,
    pub max: i8
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
    let successes: Vec<Vec<Guess>> = words.iter().map(|word| solve_for(word)).filter(is_success).collect();
    let success_count = successes.len();
    
    let percent_solved = (success_count as f32 / words.len() as f32) * 100 as f32;
    let mut guess_counts: Vec<i8> = successes.iter().map(|guesses| guesses.len() as i8).collect();
    guess_counts.sort();

    let mean = guess_counts.iter().map(|i| *i as f64).sum::<f64>() / success_count as f64;
    let median = guess_counts.iter().nth(success_count / 2).unwrap();
    let min = guess_counts.iter().next().unwrap();
    let max = guess_counts.iter().last().unwrap();


    Analysis{ percent_solved: percent_solved, mean: mean as f32, median: *median, min: *min, max: *max}
}
