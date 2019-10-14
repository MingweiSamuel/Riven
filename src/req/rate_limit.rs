use std::cmp;
use std::time::{
    Duration,
    Instant,
};

use parking_lot::{
    RwLock,
};

use super::token_bucket::{
    TokenBucket,
    VectorTokenBucket,
};
use super::rate_limit_type::RateLimitType;

pub struct RateLimit {
    rate_limit_type: RateLimitType,
    // Buckets for this rate limit (synchronized).
    // Almost always read, written only when rate limit rates are updated
    // from API response.
    // TODO: Question of writer starvation.
    buckets: RwLock<Vec<VectorTokenBucket>>,
    // Set to when we can retry if a retry-after header is received.
    retry_after: Option<Instant>,
}

impl RateLimit {
    /// Header specifying which RateLimitType caused a 429.
    const HEADER_XRATELIMITTYPE: &'static str = "X-Rate-Limit-Type";
    /// Header specifying retry after time in seconds after a 429.
    const HEADER_RETRYAFTER: &'static str = "Retry-After";

    pub fn new(rate_limit_type: RateLimitType) -> Self {
        let initial_bucket = VectorTokenBucket::new(Duration::from_secs(1), 1);
        RateLimit {
            rate_limit_type: rate_limit_type,
            // Rate limit before getting from response: 1/s.
            buckets: RwLock::new(vec![initial_bucket]),
            retry_after: None,
        }
    }

    pub fn get_both_or_delay(app_rate_limit: &Self, method_rate_limit: &Self) -> Option<Duration> {
        // Check retry after.
        let retry_after_delay = app_rate_limit.get_retry_after_delay()
            .and_then(|a| method_rate_limit.get_retry_after_delay().map(|m| cmp::max(a, m)));
        if retry_after_delay.is_some() {
            return retry_after_delay
        }
        // Check buckets.
        let app_buckets = app_rate_limit.buckets.read();
        let method_buckets = method_rate_limit.buckets.read();
        for bucket in app_buckets.iter().chain(method_buckets.iter()) {
            let delay = bucket.get_delay();
            if delay.is_some() {
                return delay;
            }
        }
        // Success.
        for bucket in app_buckets.iter().chain(method_buckets.iter()) {
            bucket.get_tokens(1);
        }
        None
    }

    pub fn get_retry_after_delay(&self) -> Option<Duration> {
        self.retry_after.and_then(|i| Instant::now().checked_duration_since(i))
    }

    pub fn on_response(&self, _response: &reqwest::Response) {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn send_sync() {
        fn is_send_sync<T: Send + Sync>() {}
        is_send_sync::<RateLimit>();
    }
}
