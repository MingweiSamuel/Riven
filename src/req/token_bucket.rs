use std::collections::VecDeque;
use std::time::{Duration, Instant};

pub trait TokenBucket {
    /// Get the duration til the next available token, or 0 duration if a token is available.
    /// # Returns
    /// Duration or 0 duration.
    fn get_delay(&mut self) -> Duration;

    /// Gets n tokens, regardless of whether they are available.
    /// # Parameters
    /// * `n` - Number of tokens to take.
    /// # Returns
    /// True if the tokens were obtained without violating limits, false otherwise.
    fn get_tokens(&mut self, n: usize) -> bool;

    /// Get the duration of this bucket.
    /// # Returns
    /// Duration of the bucket.
    fn get_bucket_duration(&self) -> Duration;

    /// Get the total limit of this bucket per timespan.
    /// # Returns
    /// Total limit per timespan.
    fn get_total_limit(&self) -> usize;
}

struct VectorTokenBucket {
    /// Duration of this TokenBucket.
    duration: Duration,
    // Total tokens available from this TokenBucket.
    total_limit: usize,
    // Record of timestamps.
    timestamps: VecDeque<Instant>,
}

impl VectorTokenBucket {
    fn create(duration: Duration, total_limit: usize) -> Self {
        VectorTokenBucket {
            duration: duration,
            total_limit: total_limit,
            timestamps: VecDeque::new(),
        }
    }

    fn update_state(&mut self) {
        let cutoff = Instant::now() - self.duration;
        while self.timestamps.back().map_or(false, |ts| ts < &cutoff) {
            self.timestamps.pop_back();
        }
    }
}

impl TokenBucket for VectorTokenBucket {

    fn get_delay(&mut self) -> Duration {
        self.update_state();
        if self.timestamps.len() < self.total_limit {
            Duration::new(0, 0)
        }
        else {
            let ts = *self.timestamps.get(self.total_limit - 1).unwrap();
            Instant::now().saturating_duration_since(ts)
        }
    }

    fn get_tokens(&mut self, n: usize) -> bool {
        self.update_state();
        let now = Instant::now();

        self.timestamps.reserve(n);
        for _ in 0..n {
            self.timestamps.push_front(now);
        }
        self.timestamps.len() <= self.total_limit
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
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
