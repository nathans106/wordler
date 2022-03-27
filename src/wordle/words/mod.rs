static UK_STRS: &str = include_str!("google-10000-english\\google-10000-english.txt");
static USA_STRS: &str = include_str!("google-10000-english\\google-10000-english-usa.txt");


pub enum Variant {
    Uk,
    Usa
}

pub type Words = Vec<&'static str>;

pub fn with_length(length: i8, variant: Variant) -> Words {
    let strs = match variant {
        Variant::Uk => UK_STRS,
        Variant::Usa => USA_STRS
    };

    strs.lines().filter(|line| line.len() == (length as usize)).collect()
}

pub fn fives(variant: Variant) -> Words {
    with_length(5, variant)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fives(){
        let words = with_length(5, Variant::Uk);
        assert!(words.len() > 0);
        words.iter().for_each(|word| assert!(word.len() == 5));
    }
}
