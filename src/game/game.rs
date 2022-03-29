use crate::words;

pub enum LetterStatus{
    Correct,
    WrongPosition,
    NotPresent
}

pub type LetterStatuses = Vec<LetterStatus>;

pub struct GameOverData {
    pub word: String,
    pub statuses: LetterStatuses
}

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
