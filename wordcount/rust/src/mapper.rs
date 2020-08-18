//! `Mapper` implementation for the wordcount project.
use efflux::prelude::{Context, Mapper};
use regex::Regex;

fn main() {
    efflux::run_mapper(WordcountMapper::new());
}

struct WordcountMapper {
    punctuation_match: Regex,
}

impl WordcountMapper {
    pub fn new() -> Self {
        Self {
            punctuation_match: Regex::new(r"[[:punct:]]|_").unwrap(),
        }
    }
}

impl Mapper for WordcountMapper {
    fn map(&mut self, _key: usize, value: &[u8], ctx: &mut Context) {
        self.punctuation_match
            .replace_all(
                &std::str::from_utf8(value).unwrap().trim().to_lowercase(),
                " ",
            )
            .split_ascii_whitespace()
            .for_each(|word| ctx.write_fmt(&word, 1));
    }
}
