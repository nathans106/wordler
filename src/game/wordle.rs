use crate::{words, game::game::{GuessResult}};

use super::{game::Game, engine::Engine};
use rand::{self, Rng};

static GUESSES: i8 = 6;
static WORD_LENGTH: i8 = 5;
static WORDS_VARIANT: words::Variant = words::Variant::Uk;

pub struct Wordle {
    engine: Engine
}

impl Wordle {
    pub fn new() -> Wordle {
        let dictionary = words::with_length(&WORD_LENGTH, &WORDS_VARIANT);
        let mut rng = rand::thread_rng();
        let word_index = rng.gen_range(0..dictionary.len());
        Wordle{engine: Engine::from(dictionary, word_index, GUESSES)}
    }
}

impl Game for Wordle {
    fn words_variant(&self) -> &words::Variant {
        &WORDS_VARIANT
    }

    fn word_length(&self) -> &i8 {
        &WORD_LENGTH
    }

    fn remaining(&self) -> &i8 {
        self.engine.remaining()
    }

    fn guess(&mut self, guess: &str) -> GuessResult {
        self.engine.guess(guess)
    }
}
