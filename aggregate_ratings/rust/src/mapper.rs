//! `Mapper` implementation for the aggregate_ratings project.
use efflux::prelude::{Context, Mapper};

fn main() {
    efflux::run_mapper(AggregateRatingsMapper::new());
}

struct AggregateRatingsMapper {
    key: usize,
}

impl AggregateRatingsMapper {
    pub fn new() -> Self {
        Self { key: 0 }
    }
}

impl Mapper for AggregateRatingsMapper {
    fn map(&mut self, _key: usize, value: &[u8], ctx: &mut Context) {
        let value = std::str::from_utf8(value).unwrap();
        if &value[value.len() - 1..] == ":" {
            self.key = value[..value.len() - 1].parse::<usize>().unwrap()
        } else {
            ctx.write_fmt(self.key, value.splitn(3, ",").nth(1).unwrap())
        }
    }
}
