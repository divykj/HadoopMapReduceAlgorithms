//! `Reducer` implementation for the aggregate_ratings project.
use efflux::prelude::{Context, Reducer};

fn main() {
    efflux::run_reducer(AggregateRatingsReducer);
}

struct AggregateRatingsReducer;

impl Reducer for AggregateRatingsReducer {
    fn reduce(&mut self, key: &[u8], values: &[&[u8]], ctx: &mut Context) {
        let num_ratings = values.len();
        let avg_rating = (values
            .iter()
            .map(|value| {
                std::str::from_utf8(value)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap()
            })
            .sum::<usize>() as f64
            / num_ratings as f64)
            .to_string();
        ctx.write(key, format!("{}\t{}", num_ratings, avg_rating).as_bytes())
    }
}
