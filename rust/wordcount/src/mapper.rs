//! `Mapper` implementation for the wordcount project.
use efflux::prelude::{Context, Mapper};
use regex::Regex;

fn main() {
    efflux::run_mapper(WordcountMapper::new());
}

struct WordcountMapper {
    word_regex: Regex,
}

impl WordcountMapper {
    pub fn new() -> Self {
        Self {
            word_regex: Regex::new(r"(\w+)").unwrap(),
        }
    }
}

impl Mapper for WordcountMapper {
    fn map(&mut self, _key: usize, value: &[u8], ctx: &mut Context) {
        self.word_regex
            .captures_iter(&std::str::from_utf8(value).unwrap().trim().to_lowercase())
            .for_each(|word| ctx.write_fmt(&word[0], 1));
    }
}
