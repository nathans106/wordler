use wordler::game::{Game, GuessResult, LetterStatuses, LetterStatus};

fn main() {
    println!("Welcome to Wordle!");
    let mut game = Game::new();
    let stdin = std::io::stdin();

    loop {
        println!("What's your guess?");
        let mut input = String::new();
        stdin.read_line(&mut input).expect("Could not read input");
        let mut guess: String = input.trim().to_string();

        while guess.len() != 5 {
            println!("Word must be 5 characters long");
            let mut input = String::new();
            stdin.read_line(&mut input).expect("Could not read input");
            guess = input.trim().to_string();
        }

        let result = game.guess(&guess);
        match result {
            GuessResult::Correct => return println!("You win!"),
            GuessResult::Incorrect(statuses) => println!(
                "{} | {} guesses remain",
                statuses_str(statuses),
                game.remaining()
            ),
            GuessResult::NotInDictionary => println!("Not in dictionary"),
            GuessResult::GameOver(data) => {
                return println!(
                    "{} | Game over, the word was {}",
                    statuses_str(data.statuses),
                    data.word
                )
            }
        }
    }
}

fn statuses_str(statuses: LetterStatuses) -> String {
    let status_char = |status: &LetterStatus| match status {
        LetterStatus::Correct => "c",
        LetterStatus::WrongPosition => "p",
        LetterStatus::NotPresent => "x",
    };

    statuses.iter().map(status_char).collect()
}
