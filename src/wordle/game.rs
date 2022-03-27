
use rand::{self, Rng};
use crate::wordle::words;

pub enum GuessResult{
    Correct,
    Incorrect,
    NotInDictionary,
    GameOver(&'static str)
}

pub struct Game {
    word_index: usize,
    dictionary: Vec<&'static str>,
    guess_count: i8
}

impl Game {
    pub fn new() -> Game {
        let dictionary = words::fives();
        let mut rng = rand::thread_rng();
        let word_index = rng.gen_range(0..dictionary.len());
        Game{word_index: word_index, dictionary: dictionary, guess_count: 0}
    }

    pub fn guess(&mut self, guess: &str) -> GuessResult {
        static MAX_GUESSES: i8 = 6;
        let word = self.dictionary[self.word_index];

        if self.guess_count >= MAX_GUESSES {
            return GuessResult::GameOver(word);
        }

        if !self.dictionary.contains(&guess){
            return GuessResult::NotInDictionary;
        }

        self.guess_count += 1;

        if guess == word{
            return GuessResult::Correct;
        }

        if self.guess_count == MAX_GUESSES{
            return GuessResult::GameOver(word);
        }

        return GuessResult::Incorrect;
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    fn make_game()-> Game {
        Game{dictionary: ["train", "apple", "house", "there", "build", "coach", "tread"].to_vec(), word_index: 1, guess_count: 0}
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