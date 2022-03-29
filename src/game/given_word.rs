use crate::words;

use super::{Game, engine::Engine, GuessResult};
pub struct GivenWord {
    engine: Engine,
    variant: words::Variant,
    word_length: i8
}

#[derive(Debug)]
pub struct NotInDictionary;

impl GivenWord {
    pub fn from(word: String, variant: words::Variant, guesses: i8) -> Result<GivenWord, NotInDictionary> {
        let word_length = word.len() as i8;
        let dictionary = words::with_length(&word_length, &variant);
        let word_index = dictionary.iter().position(|w| w==&word.as_str()).ok_or(NotInDictionary)?;
        Ok(GivenWord { engine: Engine::from(dictionary, word_index, guesses), variant: variant, word_length: word_length })
    }
}

impl Game for GivenWord {
    fn words_variant(&self) -> &words::Variant {
        &self.variant
    }

    fn word_length(&self) -> &i8 {
        &self.word_length
    }

    fn remaining(&self) -> &i8 {
        &self.engine.remaining()
    }

    fn guess(&mut self, guess: &str) -> GuessResult {
        self.engine.guess(guess)
    }
}