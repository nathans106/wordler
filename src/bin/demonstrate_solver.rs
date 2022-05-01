use wordler::{game::{Wordle, GuessResult}, solver::{RandomValidWord, Solver}};

fn main() {
    let mut game = Wordle::new();
    let guesses = RandomValidWord::solve(&mut game);
    guesses.iter().for_each(|guess| println!("{}", guess.word));
    match &guesses.iter().last().unwrap().result {
        GuessResult::Correct => println!("Success! Took {} guesses", guesses.len()),
        GuessResult::Incorrect(_) => panic!(),
        GuessResult::NotInDictionary => panic!(),
        GuessResult::GameOver(data) => println!("Failed to guess {}", data.word),
    }
}