pub mod generator;
pub mod sf;

mod config;
mod utils;

use std::sync::{Arc, Mutex};

pub struct Sf(u64);

struct GeneratorState {
    seq: u16, // sequence in milliseconds
    ts: u64,  // timestamp in milliseconds
}

pub struct Generator {
    epoch: u128,    // time offset from start of unix time in milliseconds
    worker_id: u16, // 10 bits worker id
    state: Arc<Mutex<GeneratorState>>, // shared state
}

#[cfg(test)]
mod tests {
    use super::{Generator, Sf};

    #[test]
    fn test_bad_worker_id() {
        let maybe_id = Sf::new(0, 1024, 0);
        assert!(maybe_id.is_err(), "id generating should be failed");
    }

    #[test]
    fn test_sf_nil() {
        let id = Sf::nil();

        assert_eq!(id.0, 0);
    }

    #[test]
    fn test_generate_id() {
        let gen = Generator::new(1_548_067_209_841, 0).unwrap();
        let id = gen.next_id().unwrap();

        assert_ne!(id.0, 0);
    }
}
