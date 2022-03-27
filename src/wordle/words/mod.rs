pub type Words = Vec<&'static str>;

pub fn with_length(length: i8) -> Words {  
    let words_str = include_str!("english-words/words_alpha.txt");
    words_str.lines().filter(|line| line.len() == (length as usize)).collect()
}

pub fn fives() -> Words {
    with_length(5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fives(){
        let words = with_length(5);
        assert!(words.len() > 0);
        words.iter().for_each(|word| assert!(word.len() == 5));
    }
}
