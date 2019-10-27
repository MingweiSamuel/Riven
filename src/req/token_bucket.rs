use std::collections::VecDeque;
use std::time::Duration;

use parking_lot::{Mutex, MutexGuard};

use super::Instant; // Hack for token_bucket_test.rs.

/// A `TokenBucket` keeps track of number of requests allowed per duration of
/// time.
///
/// Respone headers contain descriptions of rate limits such as
/// `"X-App-Rate-Limit": "20:1,100:120"`. Each `TokenBucket` corresponds to a
/// single `"100:120"` (100 requests per 120 seconds).
pub trait TokenBucket {
    /// Get the duration til the next available token, or 0 duration if a token
    /// is available.
    /// # Returns
    /// Duration or 0 duration.
    fn get_delay(&self) -> Option<Duration>;

    /// Gets n tokens, regardless of whether they are available.
    /// # Parameters
    /// * `n` - Number of tokens to take.
    /// # Returns
    /// True if the tokens were obtained without violating limits, false
    /// otherwise.
    fn get_tokens(&self, n: usize) -> bool;

    /// Get the duration of this bucket.
    /// # Returns
    /// Duration of the bucket.
    fn get_bucket_duration(&self) -> Duration;

    /// Get the total limit of this bucket per timespan.
    /// # Returns
    /// Total limit per timespan.
    fn get_total_limit(&self) -> usize;
}

#[derive(Debug)]
pub struct VectorTokenBucket {
    /// Duration of this TokenBucket.
    duration: Duration,
    // Total tokens available from this TokenBucket.
    total_limit: usize,

    /// TODO USE THESE !!!!!!!
    /// Duration considered for burst factor.
    burst_duration: Duration,
    /// Limit allowed per burst_duration, for burst factor.
    burst_limit: usize,

    /// Record of timestamps (synchronized).
    timestamps: Mutex<VecDeque<Instant>>,
}

impl VectorTokenBucket {
    pub fn new(duration: Duration, total_limit: usize, burst_pct: f32) -> Self {
        debug_assert!(0.0 < burst_pct && burst_pct <= 1.0,
            "BAD burst_pct {}.", burst_pct);
        // Float ops may lose precision, but nothing should be that precise.
        // API always uses round numbers, burst_pct is frac of 256.

        let burst_duration = Duration::new(
            (duration.as_secs()      as f32 * burst_pct) as u64,
            (duration.subsec_nanos() as f32 * burst_pct) as u32);

        VectorTokenBucket {
            duration: duration,
            total_limit: total_limit,

            burst_duration: burst_duration,
            burst_limit: (total_limit as f32 * burst_pct) as usize,

            timestamps: Mutex::new(VecDeque::new()),
        }
    }

    fn update_get_timestamps(&self) -> MutexGuard<VecDeque<Instant>> {
        let mut timestamps = self.timestamps.lock();
        let cutoff = Instant::now() - self.duration;
        while timestamps.back().map_or(false, |ts| ts < &cutoff) {
            timestamps.pop_back();
        }
        return timestamps;
    }
}

impl TokenBucket for VectorTokenBucket {

    fn get_delay(&self) -> Option<Duration> {
        let timestamps = self.update_get_timestamps();

        // The "?" means:
        // `if timestamps.len() < self.total_limit { return None }`
        // Timestamp that needs to be popped before
        // we can enter another timestamp.
        let ts = *timestamps.get(self.total_limit - 1)?;
        Instant::now().checked_duration_since(ts)
            .and_then(|passed_dur| self.duration.checked_sub(passed_dur))
    }

    fn get_tokens(&self, n: usize) -> bool {
        let mut timestamps = self.update_get_timestamps();

        let now = Instant::now();

        timestamps.reserve(n);
        for _ in 0..n {
            timestamps.push_front(now);
        }
        timestamps.len() <= self.total_limit
    }

    fn get_bucket_duration(&self) -> Duration {
        self.duration
    }

    fn get_total_limit(&self) -> usize {
        self.total_limit
    }
}
