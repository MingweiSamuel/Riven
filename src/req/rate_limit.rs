use std::time::Duration;

pub trait RateLimit {
    fn get_retry_after_delay(&self) -> Duration;
}

