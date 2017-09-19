use rand::{Rng, SeedableRng, thread_rng};
use rand::isaac::Isaac64Rng;

pub fn make_random(seed : u64) -> Box<Rng> {
    Box::new(Isaac64Rng::from_seed(&[seed]))
}

pub fn make_seed() -> u64 {
    thread_rng().next_u64()
}