//! Configuration of RiotApi.
use std::time::Duration;

use reqwest::ClientBuilder;
use reqwest::header::{ HeaderMap, HeaderValue };

/// Configuration for instantiating RiotApi.
///
///
#[derive(Debug)]
pub struct RiotApiConfig {
    pub(crate) base_url: String,
    pub(crate) retries: u8,
    pub(crate) app_rate_usage_factor: f32,
    pub(crate) method_rate_usage_factor: f32,
    pub(crate) burst_factor: f32,
    pub(crate) duration_overhead: Duration,
    pub(crate) client_builder: Option<ClientBuilder>,
}

impl RiotApiConfig {
    /// Request header name for the Riot API key.
    ///
    /// When using `set_client_builder`, the supplied builder should include
    /// this default header with the Riot API key as the value.
    const RIOT_KEY_HEADER: &'static str = "X-Riot-Token";

    /// `"https://{}.api.riotgames.com"`
    ///
    /// Default base URL, including `{}` placeholder for region platform.
    pub const DEFAULT_BASE_URL: &'static str = "https://{}.api.riotgames.com";

    /// `3`
    ///
    /// Default number of retries.
    pub const DEFAULT_RETRIES: u8 = 3;

    /// `1.0`
    ///
    /// Default rate limit usage factor.
    pub const DEFAULT_RATE_USAGE_FACTOR: f32 = 1.0;

    /// `0.99`
    ///
    /// Default `burst_factor`, also used by `preconfig_burst`.
    pub const PRECONFIG_BURST_BURST_FACTOR: f32 = 0.99;
    /// `989` ms
    ///
    /// Default `duration_overhead`, also used by `preconfig_burst`.
    pub const PRECONFIG_BURST_DURATION_OVERHEAD: Duration = Duration::from_millis(989);

    /// `0.47`
    ///
    /// `burst_factor` used by `preconfig_throughput`.
    pub const PRECONFIG_THROUGHPUT_BURST_FACTOR: f32 = 0.47;
    /// `10` ms.
    ///
    /// `duration_overhead` used by `preconfig_throughput`.
    pub const PRECONFIG_THROUGHPUT_DURATION_OVERHEAD: Duration = Duration::from_millis(10);

    /// Creates a new `RiotApiConfig` with the given `api_key` with the following
    /// configuration:
    ///
    /// * `retries = 3` (`RiotApiConfig::DEFAULT_RETRIES`).
    /// * `burst_factor = 0.99` (`preconfig_burst`).
    /// * `duration_overhead = 989 ms` (`preconfig_burst`).
    ///
    /// `api_key` should be a Riot Games API key from
    /// [https://developer.riotgames.com/](https://developer.riotgames.com/),
    /// and should look like `"RGAPI-01234567-89ab-cdef-0123-456789abcdef"`.
    pub fn with_key(api_key: impl AsRef<[u8]>) -> Self {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            Self::RIOT_KEY_HEADER,
            HeaderValue::from_bytes(api_key.as_ref()).unwrap()
        );

        Self {
            base_url: Self::DEFAULT_BASE_URL.into(),
            retries: Self::DEFAULT_RETRIES,
            app_rate_usage_factor: Self::DEFAULT_RATE_USAGE_FACTOR,
            method_rate_usage_factor: Self::DEFAULT_RATE_USAGE_FACTOR,
            burst_factor: Self::PRECONFIG_BURST_BURST_FACTOR,
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
    /// * `retries = 3` (`RiotApiConfig::DEFAULT_RETRIES`).
    /// * `burst_factor = 0.99` (`preconfig_burst`).
    /// * `duration_overhead = 989 ms` (`preconfig_burst`).
    pub fn with_client_builder(client_builder: ClientBuilder) -> Self {
        Self {
            base_url: Self::DEFAULT_BASE_URL.to_owned(),
            retries: Self::DEFAULT_RETRIES,
            app_rate_usage_factor: Self::DEFAULT_RATE_USAGE_FACTOR,
            method_rate_usage_factor: Self::DEFAULT_RATE_USAGE_FACTOR,
            burst_factor: Self::PRECONFIG_BURST_BURST_FACTOR,
            duration_overhead: Self::PRECONFIG_BURST_DURATION_OVERHEAD,
            client_builder: Some(client_builder),
        }
    }

    /// Sets rate limiting settings to preconfigured values optimized for burst,
    /// low latency:
    ///
    /// * `burst_factor = 0.99` (`PRECONFIG_BURST_BURST_FACTOR`).
    /// * `duration_overhead = 989 ms` (`PRECONFIG_BURST_DURATION_OVERHEAD_MILLIS`).
    ///
    /// # Returns
    /// `self`, for chaining.
    pub fn preconfig_burst(mut self) -> Self {
        self.burst_factor = Self::PRECONFIG_BURST_BURST_FACTOR;
        self.duration_overhead = Self::PRECONFIG_BURST_DURATION_OVERHEAD;
        self
    }

    /// Sets the rate limiting settings to preconfigured values  optimized for
    /// high throughput:
    ///
    /// * `burst_factor = 0.47` (`PRECONFIG_THROUGHPUT_BURST_FACTOR`).
    /// * `duration_overhead = 10 ms` (`PRECONFIG_THROUGHPUT_DURATION_OVERHEAD_MILLIS`).
    ///
    /// # Returns
    /// `self`, for chaining.
    pub fn preconfig_throughput(mut self) -> Self {
        self.burst_factor = Self::PRECONFIG_THROUGHPUT_BURST_FACTOR;
        self.duration_overhead = Self::PRECONFIG_THROUGHPUT_DURATION_OVERHEAD;
        self
    }

    /// Set the base url for requests. The string should contain a `"{}"`
    /// literal which will be replaced with the region platform name. (However
    /// multiple or zero `"{}"`s may be included if needed).
    ///
    /// # Returns
    /// `self`, for chaining.
    pub fn set_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
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

    /// The rate limit usage percentage controls how much of the API key's rate
    /// limit will be used. The default value of `1.0` means the entirety of
    /// the rate limit may be used if it is needed. This applies to both the
    /// API key's rate limit (per route) _and_ to endpoint method rate limits.
    ///
    /// Setting a value lower than `1.0` can be useful if you are running
    /// multiple API instances on the same API key.
    ///
    /// For example, four instances, possibly running on different machines,
    /// could each have a value of `0.25` to share an API key's rate limit
    /// evenly.
    ///
    /// Note that if you have multiple instances hitting _different_ methods,
    /// you should use [Self::set_app_rate_usage_factor()] and [Self::set_method_rate_usage_factor()]
    /// separately, as this sets both.
    ///
    /// This also can be used to reduce the chance of hitting 429s, although
    /// 429s should be rare even with this set to `1.0`.
    ///
    /// # Panics
    /// If `rate_usage_factor` is not in range (0, 1].
    ///
    /// # Returns
    /// `self`, for chaining.
    pub fn set_rate_usage_factor(mut self, rate_usage_factor: f32) -> Self {
        // Use inverted check to handle NaN.
        if 0.0 < rate_usage_factor && rate_usage_factor <= 1.0 {
            self.app_rate_usage_factor = rate_usage_factor;
            self.method_rate_usage_factor = rate_usage_factor;
            return self;
        }
        panic!("rate_usage_factor \"{}\" not in range (0, 1].", rate_usage_factor);
    }

    /// See [Self::set_rate_usage_factor]. Setting this is useful if you have multiple
    /// instances sharing the app rate limit, but are hitting distinct methods
    /// and therefore do not need their method usage decreased.
    ///
    /// # Panics
    /// If `app_rate_usage_factor` is not in range (0, 1\].
    ///
    /// # Returns
    /// `self`, for chaining.
    pub fn set_app_rate_usage_factor(mut self, app_rate_usage_factor: f32) -> Self {
        // Use inverted check to handle NaN.
        if 0.0 < app_rate_usage_factor && app_rate_usage_factor <= 1.0 {
            self.app_rate_usage_factor = app_rate_usage_factor;
            return self;
        }
        panic!("app_rate_usage_factor \"{}\" not in range (0, 1].", app_rate_usage_factor);
    }

    /// See [Self::set_rate_usage_factor] and [Self::set_app_rate_usage_factor].
    /// This method is mainly provided for completeness, though it may be
    /// useful in advanced use cases.
    ///
    /// # Panics
    /// If `method_rate_usage_factor` is not in range (0, 1\].
    ///
    /// # Returns
    /// `self`, for chaining.
    pub fn set_method_rate_usage_factor(mut self, method_rate_usage_factor: f32) -> Self {
        // Use inverted check to handle NaN.
        if 0.0 < method_rate_usage_factor && method_rate_usage_factor <= 1.0 {
            self.method_rate_usage_factor = method_rate_usage_factor;
            return self;
        }
        panic!("method_rate_usage_factor \"{}\" not in range (0, 1].", method_rate_usage_factor);
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
    /// If `burst_factor` is not in range (0, 1\].
    ///
    /// # Returns
    /// `self`, for chaining.
    pub fn set_burst_factor(mut self, burst_factor: f32) -> Self {
        // Use inverted check to handle NaN.
        if 0.0 < burst_factor && burst_factor <= 1.0 {
            self.burst_factor = burst_factor;
            return self;
        }
        panic!("burst_factor \"{}\" not in range (0, 1].", burst_factor);
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
