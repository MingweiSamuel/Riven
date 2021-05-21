//! Configuration of RiotApi.
use std::time::Duration;

use reqwest::ClientBuilder;
use reqwest::header::{HeaderMap, HeaderValue};

/// Configuration for instantiating RiotApi.
///
///
#[derive(Debug)]
pub struct RiotApiConfig {
    pub(crate) retries: u8,
    pub(crate) burst_pct: f32,
    pub(crate) duration_overhead: Duration,
    pub(crate) client_builder: Option<ClientBuilder>,
}

impl RiotApiConfig {
    /// Request header name for the Riot API key.
    ///
    /// When using `set_client_builder`, the supplied builder should include
    /// this default header with the Riot API key as the value.
    const RIOT_KEY_HEADER: &'static str = "X-Riot-Token";

    /// `3`
    ///
    /// Default number of retries.
    pub const PRECONFIG_RETRIES: u8 = 3;

    /// `0.99`
    ///
    /// Default `burst_pct`, also used by `preconfig_burst`.
    pub const PRECONFIG_BURST_BURST_PCT: f32 = 0.99;
    /// `989` ms
    ///
    /// Default `duration_overhead`, also used by `preconfig_burst`.
    pub const PRECONFIG_BURST_DURATION_OVERHEAD: Duration = Duration::from_millis(989);

    /// `0.47`
    ///
    /// `burst_pct` used by `preconfig_throughput`.
    pub const PRECONFIG_THROUGHPUT_BURST_PCT: f32 = 0.47;
    /// `10` ms.
    ///
    /// `duration_overhead` used by `preconfig_throughput`.
    pub const PRECONFIG_THROUGHPUT_DURATION_OVERHEAD: Duration = Duration::from_millis(10);

    /// Creates a new `RiotApiConfig` with the given `api_key` with the following
    /// configuration:
    ///
    /// * `retries = 3` (`RiotApiConfig::PRECONFIG_RETRIES`).
    /// * `purst_pct = 0.99` (`preconfig_burst`).
    /// * `duration_overhead = 989 ms` (`preconfig_burst`).
    ///
    /// `api_key` should be a Riot Games API key from
    /// [https://developer.riotgames.com/](https://developer.riotgames.com/),
    /// and should look like `"RGAPI-01234567-89ab-cdef-0123-456789abcdef"`.
    pub fn with_key<T: AsRef<[u8]>>(api_key: T) -> Self {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            Self::RIOT_KEY_HEADER,
            HeaderValue::from_bytes(api_key.as_ref()).unwrap()
        );

        Self {
            retries: Self::PRECONFIG_RETRIES,
            burst_pct: Self::PRECONFIG_BURST_BURST_PCT,
            duration_overhead: Self::PRECONFIG_BURST_DURATION_OVERHEAD,
            client_builder: Some(
                ClientBuilder::new()
                    .default_headers(default_headers)
            ),
        }
    }

    /// Creates a new `RiotApiConfig` with the given client builder.
    ///
    /// The client builder default headers should include a value for
    /// `RiotApiConfig::RIOT_KEY_HEADER`, otherwise authentication will fail.
    ///
    /// * `retries = 3` (`RiotApiConfig::PRECONFIG_RETRIES`).
    /// * `purst_pct = 0.99` (`preconfig_burst`).
    /// * `duration_overhead = 989 ms` (`preconfig_burst`).
    pub fn with_client_builder(client_builder: ClientBuilder) -> Self {
        Self {
            retries: Self::PRECONFIG_RETRIES,
            burst_pct: Self::PRECONFIG_BURST_BURST_PCT,
            duration_overhead: Self::PRECONFIG_BURST_DURATION_OVERHEAD,
            client_builder: Some(client_builder),
        }
    }

    /// Sets rate limiting settings to preconfigured values optimized for burst,
    /// low latency:
    ///
    /// * `burst_pct = 0.99` (`PRECONFIG_BURST_BURST_PCT`).
    /// * `duration_overhead = 989 ms` (`PRECONFIG_BURST_DURATION_OVERHEAD_MILLIS`).
    ///
    /// # Returns
    /// `self`, for chaining.
    pub fn preconfig_burst(mut self) -> Self {
        self.burst_pct = Self::PRECONFIG_BURST_BURST_PCT;
        self.duration_overhead = Self::PRECONFIG_BURST_DURATION_OVERHEAD;
        self
    }

    /// Sets the rate limiting settings to preconfigured values  optimized for
    /// high throughput:
    ///
    /// * `burst_pct = 0.47` (`PRECONFIG_THROUGHPUT_BURST_PCT`).
    /// * `duration_overhead = 10 ms` (`PRECONFIG_THROUGHPUT_DURATION_OVERHEAD_MILLIS`).
    ///
    /// # Returns
    /// `self`, for chaining.
    pub fn preconfig_throughput(mut self) -> Self {
        self.burst_pct = Self::PRECONFIG_THROUGHPUT_BURST_PCT;
        self.duration_overhead = Self::PRECONFIG_THROUGHPUT_DURATION_OVERHEAD;
        self
    }

    /// Set number of times to retry requests. Naturally, only retryable requests
    /// will be retried: responses with status codes 5xx or 429 (after waiting
    /// for retry-after headers). A value of `0` means one request will be sent
    /// and it will not be retried if it fails.
    ///
    /// # Returns
    /// `self`, for chaining.
    pub fn set_retries(mut self, retries: u8) -> Self {
        self.retries = retries;
        self
    }

    /// Burst percentage controls how many burst requests are allowed and
    /// therefore how requests are spread out. Higher equals more burst,
    /// less spread. Lower equals less burst, more spread.
    ///
    /// The value must be in the range (0, 1];
    /// Between 0, exclusive, and 1, inclusive. However values should generally
    /// be larger than 0.25.
    ///
    /// Burst percentage behaves as follows:<br>
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
    /// If `burst_pct` is not in range (0, 1].
    ///
    /// # Returns
    /// `self`, for chaining.
    pub fn set_burst_pct(mut self, burst_pct: f32) -> Self {
        // Use inverted check to handle NaN.
        if 0.0 < burst_pct && burst_pct < 1.0 {
            self.burst_pct = burst_pct;
            return self;
        }
        panic!("burst_pct \"{}\" not in range (0, 1].", burst_pct);
    }

    /// Sets the additional bucket duration to consider when rate limiting.
    /// Increasing this value will decrease the chances of 429s, but will lower
    /// the overall throughput.
    ///
    /// In a sense, the `duration_overhead` is how much to "widen" the temporal
    /// width of buckets.
    ///
    /// Given a particular Riot Game API rate limit bucket that allows N requests
    /// per D duration, when counting requests this library will consider requests
    /// sent in the past `D + duration_overhead` duration.
    ///
    /// # Returns
    /// `self`, for chaining.
    pub fn set_duration_overhead(mut self, duration_overhead: Duration) -> Self {
        self.duration_overhead = duration_overhead;
        self
    }
}
