//! `Reducer` implementation for the wordcount project.
extern crate efflux;

use efflux::prelude::{Context, Reducer};

fn main() {
    efflux::run_reducer(WordcountReducer);
}

struct WordcountReducer;

impl Reducer for WordcountReducer {
    fn reduce(&mut self, key: &[u8], values: &[&[u8]], ctx: &mut Context) {
        ctx.write(
            key,
            values
                .iter()
                .map(|value| {
                    std::str::from_utf8(value)
                        .unwrap()
                        .parse::<usize>()
                        .unwrap()
                })
                .sum::<usize>()
                .to_string()
                .as_bytes(),
        );
    }
}
