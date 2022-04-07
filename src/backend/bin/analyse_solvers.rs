use wordler::solver::{analyser::analyse, RandomValidWord};
use wordler::game::wordle;
use wordler::words;

fn main() {
    println!("Analysing random valid word");
    let analysis = analyse::<RandomValidWord>(&wordle::WORD_LENGTH, &words::Variant::Uk, wordle::GUESSES);
    println!("{:.2}% solved", analysis.percent_solved);
    println!("Mean: {:.2}", analysis.mean);
    println!("Median: {}", analysis.median);
    println!("Range: {} - {}", analysis.min, analysis.max);
}