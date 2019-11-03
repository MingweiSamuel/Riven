use std::fmt;
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
    fn get_bucket_duration(&self) -> Duration;

    /// Get the total limit of this bucket per timespan.
    /// # Returns
    /// Total limit per timespan.
    fn get_total_limit(&self) -> usize;
}

pub struct VectorTokenBucket {
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
    pub fn new(duration: Duration, total_limit: usize,
        duration_overhead: Duration, burst_pct: f32) -> Self
    {
        debug_assert!(0.0 < burst_pct && burst_pct <= 1.0,
            "BAD burst_pct {}.", burst_pct);
        // Float ops may lose precision, but nothing should be that precise.
        // API always uses round numbers, burst_pct is frac of 256.

        // Effective duration.
        let d_eff = duration + duration_overhead;
        let burst_duration = Duration::new(
            (d_eff.as_secs()      as f32 * burst_pct).ceil()  as u64,
            (d_eff.subsec_nanos() as f32 * burst_pct).ceil()  as u32);
        let burst_limit = std::cmp::max(1,
            (total_limit          as f32 * burst_pct).floor() as usize);
        debug_assert!(burst_limit <= total_limit);

        VectorTokenBucket {
            duration: duration,
            total_limit: total_limit,
            duration_overhead: duration_overhead,

            burst_duration: burst_duration,
            burst_limit: burst_limit,

            timestamps: Mutex::new(VecDeque::with_capacity(total_limit)),
        }
    }

    fn update_get_timestamps(&self) -> MutexGuard<VecDeque<Instant>> {
        let mut timestamps = self.timestamps.lock();
        let cutoff = Instant::now() - self.duration - self.duration_overhead;
        // We only need to trim the end of the queue to not leak memory.
        // We could do it lazily somehow if we wanted to be really fancy.
        while timestamps.back().map_or(false, |ts| *ts < cutoff) {
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

        // Full rate limit.
        if let Some(ts) = timestamps.get(self.total_limit - 1) {
            // Return amount of time needed for timestamp `ts` to go away.
            Instant::now().checked_duration_since(*ts)
                .and_then(|passed_dur| (self.duration + self.duration_overhead)
                    .checked_sub(passed_dur))
        }
        // Otherwise burst rate limit.
        else if let Some(ts) = timestamps.get(self.burst_limit - 1) {
            // Return amount of time needed for timestamp `ts` to go away.
            Instant::now().checked_duration_since(*ts)
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
        timestamps.len() <= self.total_limit
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
        write!(f, "({}/{}:{})", self.timestamps.lock().len(), self.total_limit, self.duration.as_secs())
    }
}
