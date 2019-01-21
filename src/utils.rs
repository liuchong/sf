use std::time::{SystemTime, UNIX_EPOCH};

pub fn ms_since_epoch() -> Result<u128, String> {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(since_epoch) => Ok(u128::from(since_epoch.as_secs()) * 1000
            + u128::from(since_epoch.subsec_millis())),
        Err(_) => Err("Time went backwards".to_string()),
    }
}
