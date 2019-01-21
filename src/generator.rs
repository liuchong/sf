use super::config::MAX_SEQ;
use super::utils::ms_since_epoch;
use super::{Generator, GeneratorState, Sf};
use std::sync::{Arc, Mutex};

impl Generator {
    pub fn new(epoch: u128, worker_id: u16) -> Result<Generator, String> {
        let now = ms_since_epoch()?;

        if now < epoch {
            return Err(format!("bad epoch {}", epoch));
        }

        Ok(Generator {
            epoch,
            worker_id,
            state: Arc::new(Mutex::new(GeneratorState { seq: 0, ts: 0 })),
        })
    }

    pub fn next_id(&self) -> Result<Sf, String> {
        let mut state = self.state.lock().unwrap();
        let ts = self.get_ts()?;

        if ts == state.ts {
            (*state).seq += 1;
        } else {
            (*state).seq = 0;
            (*state).ts = ts;
        };

        let seq = state.seq;

        if seq >= MAX_SEQ {
            return Err(format!("bad seq {}", seq));
        }

        Sf::new(ts, self.worker_id, seq)
    }

    fn get_ts(&self) -> Result<u64, String> {
        let now = ms_since_epoch()?;
        Ok((now - self.epoch) as u64)
    }
}
