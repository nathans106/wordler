use crate::game::{Game, GuessResult};

pub struct Guess {
    pub word: String,
    pub result: GuessResult
}

pub trait Solver {
    fn solve(game: &mut dyn Game) -> Vec<Guess>;
}
