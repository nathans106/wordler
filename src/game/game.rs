use crate::words;

#[derive(Clone, Copy)]
pub enum LetterStatus{
    Correct,
    WrongPosition,
    NotPresent
}

pub type LetterStatuses = Vec<LetterStatus>;

#[derive(Clone)]
pub struct GameOverData {
    pub word: String,
    pub statuses: LetterStatuses
}

#[derive(Clone)]
pub enum GuessResult{
    Correct,
    Incorrect(LetterStatuses),
    NotInDictionary,
    GameOver(GameOverData)
}

pub trait Game {
    fn words_variant(&self) -> &words::Variant;
    fn word_length(&self) -> &i8;
    fn remaining(&self) -> &i8;
    fn guess(&mut self, guess: &str) -> GuessResult;
}
