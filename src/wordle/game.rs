
use rand::{self, Rng};
use crate::wordle::words;

pub enum GuessResult{
    Correct,
    Incorrect,
    NotInDictionary,
    GameOver(&'static str)
}

pub struct Game {
    dictionary: Vec<&'static str>,
    remaining_guesses: i8,
    word_index: usize
}

impl Game {
    pub fn new() -> Game {
        let dictionary = words::fives();
        let mut rng = rand::thread_rng();
        let word_index = rng.gen_range(0..dictionary.len());
        Game{word_index: word_index, dictionary: dictionary, remaining_guesses: 6}
    }

    pub fn guess(&mut self, guess: &str) -> GuessResult {
        assert!(guess.len() == 5);
        let word = self.dictionary[self.word_index];

        if self.remaining_guesses <= 0 {
            return GuessResult::GameOver(word);
        }

        if !self.dictionary.contains(&guess){
            return GuessResult::NotInDictionary;
        }

        self.remaining_guesses -= 1;

        if guess == word{
            return GuessResult::Correct;
        }

        if self.remaining_guesses <= 0 {
            return GuessResult::GameOver(word);
        }

        return GuessResult::Incorrect;
    }

    pub fn remaining(&self) -> i8 {
        self.remaining_guesses
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    fn make_game()-> Game {
        Game{dictionary: ["train", "apple", "house", "there", "build", "coach", "tread"].to_vec(), word_index: 1, remaining_guesses: 6}
    }

    #[test]
    fn test_game_correct(){
        let mut game = make_game();
        assert!(matches!(game.guess("apple"), GuessResult::Correct));
    }

    #[test]
    fn test_game_incorrect() {
        let mut game = make_game();
        assert!(matches!(game.guess("there"), GuessResult::Incorrect));
    }

    #[test]
    fn test_game_not_in_dictionary() {
        let mut game = make_game();
        assert!(matches!(game.guess("paste"), GuessResult::NotInDictionary));
    }

    #[test]
    fn test_game_game_over() {
        let mut game = make_game();
        assert!(matches!(game.guess("there"), GuessResult::Incorrect));
        assert!(matches!(game.guess("there"), GuessResult::Incorrect));
        assert!(matches!(game.guess("there"), GuessResult::Incorrect));
        assert!(matches!(game.guess("there"), GuessResult::Incorrect));
        assert!(matches!(game.guess("there"), GuessResult::Incorrect));
        assert!(matches!(game.guess("there"), GuessResult::GameOver("apple")));
    }
}