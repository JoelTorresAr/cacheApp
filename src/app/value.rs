use std::time::{Duration, SystemTime};
#[derive(Debug)]
pub struct CacheValue {
    pub expires_at: Option<SystemTime>,
    pub value: String,
    pub group: Option<String>
}

impl CacheValue {
    pub fn new(value: String, hours: Option<u64>, group: Option<String>) -> Self {
        let expires_at = match hours {
            Some(hours) => Some(SystemTime::now() + Duration::from_secs(hours * 60 * 60)),
            None => None,
        };
        CacheValue {
            expires_at,
            value,
            group
        }
    }
}