use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_current_time() -> u64 {
    let system_time = SystemTime::now();
    let seconds = system_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
    return seconds;
}