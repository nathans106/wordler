use crate::game::GameOverData;

use super::{LetterStatuses, LetterStatus, GuessResult};

pub struct Engine {
    dictionary: Vec<&'static str>,
    remaining_guesses: i8,
    word_index: usize
}

impl Engine {
    pub fn from(dictionary: Vec<&'static str>, word_index: usize, guesses: i8) -> Engine {
        Engine{dictionary: dictionary, word_index: word_index, remaining_guesses: guesses}
    }

    pub fn remaining(&self) -> &i8 {
        &self.remaining_guesses
    }

    pub fn word(&self) -> String {
        self.dictionary[self.word_index].to_string()
    }

    pub fn guess(&mut self, guess: &str) -> GuessResult {
        assert!(self.remaining_guesses > 0);
        assert!(guess.len() == 5);

        if !self.dictionary.contains(&guess){
            return GuessResult::NotInDictionary;
        }

        self.remaining_guesses -= 1;
        let word = self.word();

        if guess == word{
            return GuessResult::Correct;
        }

        let statuses = self.make_statuses(guess);

        if self.remaining_guesses <= 0 {
            return GuessResult::GameOver(GameOverData{word: word, statuses: statuses});
        }

        return GuessResult::Incorrect(statuses);
    }

    fn make_statuses(&self, guess: &str) -> LetterStatuses {
        guess.chars().enumerate().map(|(pos, letter)| self.make_status(pos, letter)).collect()
    }

    fn make_status(&self, pos: usize, letter: char) -> LetterStatus {
        let word = self.word();
        if letter == word.chars().nth(pos).unwrap() {
            return LetterStatus::Correct;
        }

        if word.contains(letter) {
            return LetterStatus::WrongPosition;
        }

        return LetterStatus::NotPresent;
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    static GUESSES: i8 = 6;

    fn make_engine()-> Engine {
        Engine{dictionary: ["train", "apple", "house", "there", "build", "coach", "tread"].to_vec(), word_index: 1, remaining_guesses: GUESSES}
    }

    #[test]
    fn test_game_correct(){
        let mut game = make_engine();
        assert!(matches!(game.guess("apple"), GuessResult::Correct));
    }

    #[test]
    fn test_game_incorrect() {
        use LetterStatus::{Correct, NotPresent, WrongPosition};
        let statuses = [NotPresent, NotPresent, WrongPosition, NotPresent, Correct];

        let mut game = make_engine();
        assert!(matches!(game.guess("there"), GuessResult::Incorrect(statuses)));
    }

    #[test]
    fn test_game_not_in_dictionary() {
        let mut game = make_engine();
        // If the word isn't in the dictionary then it shouldn't count as a guess, so attempt 7 times.
        for _ in 0..(GUESSES + 1) {
            assert!(matches!(game.guess("paste"), GuessResult::NotInDictionary));
        }
    }

    #[test]
    fn test_game_game_over() {
        use LetterStatus::{Correct, NotPresent, WrongPosition};
        let statuses = [NotPresent, NotPresent, WrongPosition, NotPresent, Correct];

        let mut game = make_engine();
        for _ in 0..(GUESSES - 1) {
            assert!(matches!(game.guess("there"), GuessResult::Incorrect(statuses)));
        }

        let game_over_data = GameOverData{word: "apple".to_string(), statuses: statuses.to_vec()};
        assert!(matches!(game.guess("there"), GuessResult::GameOver(game_over_data)));
    }
}