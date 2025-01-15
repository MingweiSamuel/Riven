use std::collections::VecDeque;
use std::fmt;

use parking_lot::{Mutex, MutexGuard};

use super::Instant; // Hack for token_bucket_test.rs.
use crate::time::Duration;

/// A `TokenBucket` keeps track of number of requests allowed per duration of
/// time.
///
/// Respone headers contain descriptions of rate limits such as
/// `"X-App-Rate-Limit": "20:1,100:120"`. Each `TokenBucket` corresponds to a
/// single `"100:120"` (100 requests per 120 seconds).
pub trait TokenBucket {
    /// Get the duration til the next available token, or None if a token
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
    #[cfg_attr(feature = "nightly", allow(dead_code, reason = "false positive"))]
    #[cfg_attr(not(feature = "nightly"), allow(dead_code))]
    fn get_bucket_duration(&self) -> Duration;

    /// Get the total limit of this bucket per timespan.
    /// # Returns
    /// Total limit per timespan.
    #[cfg_attr(feature = "nightly", allow(dead_code, reason = "false positive"))]
    #[cfg_attr(not(feature = "nightly"), allow(dead_code))]
    fn get_total_limit(&self) -> usize;
}

pub struct VectorTokenBucket {
    /// The total limit supplied to the constructor, unadjusted by the [rate_usage_factor].
    _given_total_limit: usize,
    /// Additional factor to reduce rate limit usage, in range (0, 1\].
    _rate_usage_factor: f32,

    /// Duration of this TokenBucket.
    duration: Duration,
    // Total tokens available from this TokenBucket.
    total_limit: usize,

    /// Extra duration to be considered on top of `duration`, to account for
    /// varying network latency.
    duration_overhead: Duration,

    /// Duration considered for burst factor.
    burst_duration: Duration,
    /// Limit allowed per burst_duration, for burst factor.
    burst_limit: usize,

    /// Record of timestamps (synchronized).
    timestamps: Mutex<VecDeque<Instant>>,
}

impl VectorTokenBucket {
    pub fn new(
        duration: Duration,
        given_total_limit: usize,
        duration_overhead: Duration,
        burst_factor: f32,
        rate_usage_factor: f32,
    ) -> Self {
        debug_assert!(
            0.0 < rate_usage_factor && rate_usage_factor <= 1.0,
            "BAD rate_usage_factor {}.",
            rate_usage_factor
        );
        debug_assert!(
            0.0 < burst_factor && burst_factor <= 1.0,
            "BAD burst_factor {}.",
            burst_factor
        );
        // Float ops may lose precision, but nothing should be that precise.
        // API always uses round numbers, burst_factor is frac of 256.

        // Adjust everything by rate_usage_factor.
        let total_limit = std::cmp::max(
            1,
            (given_total_limit as f32 * rate_usage_factor).floor() as usize,
        );

        // Effective duration.
        let d_eff = duration + duration_overhead;
        let burst_duration = d_eff.mul_f32(burst_factor);
        let burst_limit = std::cmp::max(1, (total_limit as f32 * burst_factor).floor() as usize);
        debug_assert!(burst_limit <= total_limit);

        VectorTokenBucket {
            _given_total_limit: given_total_limit,
            _rate_usage_factor: rate_usage_factor,

            duration,
            total_limit,

            duration_overhead,
            burst_duration,
            burst_limit,

            timestamps: Mutex::new(VecDeque::with_capacity(total_limit)),
        }
    }

    fn update_get_timestamps(&self) -> MutexGuard<VecDeque<Instant>> {
        let mut timestamps = self.timestamps.lock();
        // Only `None` in wasm, for some implementation reason. Probably sets time 0 at the first
        // `Instant::now()` call or something.
        if let Some(cutoff) = Instant::now().checked_sub(self.duration + self.duration_overhead) {
            // Pop off timestamps that are beyound the bucket duration.
            while timestamps.back().is_some_and(|ts| *ts < cutoff) {
                timestamps.pop_back();
            }
        }
        timestamps
    }
}

impl TokenBucket for VectorTokenBucket {
    fn get_delay(&self) -> Option<Duration> {
        let timestamps = self.update_get_timestamps();

        // Full rate limit.
        if let Some(ts) = timestamps.get(self.total_limit - 1) {
            // Return amount of time needed for timestamp `ts` to go away.
            Instant::now()
                .checked_duration_since(*ts)
                .and_then(|passed_dur| {
                    (self.duration + self.duration_overhead).checked_sub(passed_dur)
                })
        }
        // Otherwise burst rate limit.
        else if let Some(ts) = timestamps.get(self.burst_limit - 1) {
            // Return amount of time needed for timestamp `ts` to go away.
            Instant::now()
                .checked_duration_since(*ts)
                .and_then(|passed_dur| self.burst_duration.checked_sub(passed_dur))
        }
        // No delay needed.
        else {
            None
        }
    }

    fn get_tokens(&self, n: usize) -> bool {
        let mut timestamps = self.update_get_timestamps();

        let now = Instant::now();

        timestamps.reserve(n);
        for _ in 0..n {
            timestamps.push_front(now);
        }

        // Check total limit.
        if self.total_limit < timestamps.len() {
            return false;
        }

        // Check burst limit.
        if let Some(burst_time) = timestamps.get(self.burst_limit) {
            let duration_since = now.duration_since(*burst_time); // `now` before `burst_time` will panic.
            if duration_since < self.burst_duration {
                return false;
            }
        }

        true
    }

    fn get_bucket_duration(&self) -> Duration {
        self.duration
    }

    fn get_total_limit(&self) -> usize {
        self.total_limit
    }
}

impl fmt::Debug for VectorTokenBucket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}/{}:{})",
            self.timestamps.lock().len(),
            self.total_limit,
            self.duration.as_secs()
        )
    }
}
