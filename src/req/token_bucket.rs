use std::collections::VecDeque;
use std::time::{Duration, Instant};

use parking_lot::{Mutex, MutexGuard};

pub trait TokenBucket {
    /// Get the duration til the next available token, or 0 duration if a token is available.
    /// # Returns
    /// Duration or 0 duration.
    fn get_delay(&self) -> Option<Duration>;

    /// Gets n tokens, regardless of whether they are available.
    /// # Parameters
    /// * `n` - Number of tokens to take.
    /// # Returns
    /// True if the tokens were obtained without violating limits, false otherwise.
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
    // Record of timestamps (synchronized).
    timestamps: Mutex<VecDeque<Instant>>,
}

impl VectorTokenBucket {
    pub fn new(duration: Duration, total_limit: usize) -> Self {
        VectorTokenBucket {
            duration: duration,
            total_limit: total_limit,
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

#[cfg(test)]
mod tests {
    // use super::*;
    //
    // #[test]
    // fn it_works() {
    //     assert_eq!(2 + 2, 4);
    // }
}
