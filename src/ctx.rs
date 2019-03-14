use super::defs::MAX_SEQ;
use super::utils::ms_since_epoch;
use super::Id;
use std::sync::{Arc, Mutex};

struct State {
    seq: u16, // sequence in milliseconds
    ts: u64,  // timestamp in milliseconds
}

#[derive(Clone)]
pub struct Context {
    epoch: u128,    // time offset from start of unix time in milliseconds
    worker_id: u16, // 10 bits worker id
    state: Arc<Mutex<State>>, // shared state
}

impl Context {
    pub fn new(epoch: u128, worker_id: u16) -> Result<Context, String> {
        let now = ms_since_epoch()?;

        if now < epoch {
            return Err(format!("bad epoch {}", epoch));
        }

        Ok(Context {
            epoch,
            worker_id,
            state: Arc::new(Mutex::new(State { seq: 0, ts: 0 })),
        })
    }

    pub fn next(&self) -> Result<Id, String> {
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

        Ok(Id::new(ts, self.worker_id, seq))
    }

    pub fn next_id(&self) -> Id {
        loop {
            if let Ok(id) = self.next() {
                return id;
            }
        }
    }

    fn get_ts(&self) -> Result<u64, String> {
        let now = ms_since_epoch()?;
        Ok((now - self.epoch) as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::Context;

    #[test]
    fn test_generate_id() {
        let context = Context::new(1_548_067_209_841, 0).unwrap();
        let id = context.next_id();

        assert_ne!(id, 0.into());
    }
}
