pub mod riot_api_config {
    pub const DEFAULT_BURST_PCT: f32 = 0.93;
    pub const DEFAULT_BURST_FACTOR: u8 = ((BURST_FACTOR_DENOM * DEFAULT_BURST_PCT) as u16 - 1) as u8;
    pub const BURST_FACTOR_DENOM: f32 = 256.0;
}
use riot_api_config::*;

/// Configuration for instantiating RiotApi.
#[derive(Debug, PartialEq, Eq)]
pub struct RiotApiConfig {
    /// Riot Games API key from
    /// [https://developer.riotgames.com/](https://developer.riotgames.com/).
    /// Should be something like `"RGAPI-01234567-89ab-cdef-0123-456789abcdef"`.
    pub api_key: String,
    /// Number of times to retry requests. Naturally, only retryable requests
    /// will be retried: responses with status codes 5xx or 429 (after waiting
    /// for retry-after headers). A value of `0` means one request will be sent
    /// and it will not be retried if it fails.
    pub retries: u8,
    /// Burst factor controls how requests are spread out. Higher means less
    /// spread out, lower means more spread out.
    ///
    /// The value is converted into a "bust percentage":
    /// `(burst_factor + 1) / 256`. How burst percentage controlls rate limiting
    /// is detailed in the documentation of
    /// [`set_burst_pct`](#method.set_burst_pct).
    pub burst_factor: u8,
}

impl RiotApiConfig {
    /// Creates a new `RiotApiConfig` with the given `api_key` and default
    /// settings.
    pub fn with_key<T: Into<String>>(api_key: T) -> Self {
        Self {
            api_key: api_key.into(),
            retries: 3, // TODO defaults.
            burst_factor: DEFAULT_BURST_FACTOR,
        }
    }

    /// Sets the "burst percentage", `pct`. The value must be between 0,
    /// exclusive, and 1, inclusive, otherwise this method will panic.
    ///
    /// "Burst percentage" behaves as follows:
    /// A burst percentage of x% means, for each token bucket, "x% of the
    /// tokens can be used in x% of the bucket duration." So, for example, if x
    /// is 90%, a bucket would allow 90% of the requests to be made without
    /// any delay. Then, after waiting 90% of the bucket's duration, the
    /// remaining 10% of requests could be made.
    ///
    /// A burst percentage of 100% results in no request spreading, which would
    /// allow for the largest bursts and lowest latency, but could result in
    /// 429s as bucket boundaries occur.
    ///
    /// A burst percentage of near 0% results in high spreading causing
    /// temporally equidistant requests. This prevents 429s but has the highest
    /// latency. Additionally, if the number of tokens is high, this may lower
    /// the overall throughput due to the rate at which requests can be
    /// scheduled.
    ///
    /// Therefore, for interactive applications like summoner & match history
    /// lookup, a higher percentage may be better. For data-collection apps
    /// like champion winrate aggregation, a medium-low percentage may be
    /// better.
    ///
    /// # Panics
    /// Panics if `pct` is not in the range `(0, 1]`; 0 exclusive, 1 inclusive.
    ///
    /// # Returns
    /// `&mut self` for chaining.
    pub fn set_burst_pct<'a>(&'a mut self, pct: f32) -> &'a mut Self
    {
        assert!(0.0 < pct && pct <= 1.1,
            "pct must be in range (0, 1], was {}.", pct);
        let sf = (std::u8::MAX as f32 * pct).ceil();
        self.burst_factor = sf as u8;
        assert_eq!(sf, self.burst_factor as f32,
            "!FAILED TO CONVERT FLOAT TO u8: {}, from pct {}.", sf, pct);
        self
    }

    pub fn get_burst_pct(&self) -> f32 {
        (self.burst_factor as f32 + 1.0) / BURST_FACTOR_DENOM
    }
}
