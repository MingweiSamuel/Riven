use std::future::Future;
use std::sync::Arc;

#[cfg(feature = "tracing")]
use tracing as log;

use reqwest::{Client, Method, RequestBuilder};

use crate::req::RegionalRequester;
use crate::util::InsertOnlyCHashMap;
use crate::ResponseInfo;
use crate::Result;
use crate::RiotApiConfig;
use crate::RiotApiError;

/// For retrieving data from the Riot Games API.
///
/// # Usage
///
/// Construct an instance using [`RiotApi::new(api_key or config)`](RiotApi::new).
/// The parameter may be a Riot API key string or a [`RiotApiConfig`]. Riot API
/// keys are obtained from the [Riot Developer Portal](https://developer.riotgames.com/)
/// and look like `"RGAPI-01234567-89ab-cdef-0123-456789abcdef"`.
///
/// An instance provides access to "endpoint handles" which in turn provide
/// access to individual API method calls. For example, to get a summoner by
/// name we first access the [`summoner_v4()`](RiotApi::summoner_v4) endpoints
/// then call the [`get_by_summoner_name()`](crate::endpoints::SummonerV4::get_by_summoner_name)
/// method:
/// ```ignore
/// riot_api.summoner_v4().get_by_summoner_name(Region::NA, "LugnutsK")
/// ```
///
/// # Rate Limiting
///
/// The Riot Game API enforces _dynamic_ rate limiting, meaning that rate limits are
/// specified in response headers and can change at any time.
/// Riven keeps track of changing rate limits seamlessly, preventing you from
/// getting blacklisted.
///
/// Riven's rate limiting is highly efficient; it can use the full throughput
/// of your rate limit without triggering 429 errors.
///
/// To adjust rate limiting, see [RiotApiConfig] and use
/// [`RiotApi::new(config)`](RiotApi::new) to construct an instance.
#[derive(Clone)]
pub struct RiotApi {
    /// Configuration settings.
    config: RiotApiConfig,
    /// Client for making requests.
    client: Client,

    /// Per-region requesters.
    regional_requesters: InsertOnlyCHashMap<&'static str, RegionalRequester>,
}

impl RiotApi {
    /// Constructs a new instance from an API key (e.g. `"RGAPI-01234567-89ab-cdef-0123-456789abcdef"`) or a [RiotApiConfig].
    pub fn new(config: impl Into<RiotApiConfig>) -> Self {
        let mut config = config.into();
        let client_builder = config
            .client_builder
            .take()
            .expect("CLIENT_BUILDER IN CONFIG SHOULD NOT BE NONE.");
        Self {
            config,
            client: client_builder
                .build()
                .expect("Failed to create client from builder."),
            regional_requesters: InsertOnlyCHashMap::new(),
        }
    }

    /// This method should generally not be used directly. Consider using endpoint wrappers instead.
    ///
    /// Creates a `RequestBuilder` instance with the given parameters, for use with the `execute*()` methods.
    ///
    /// # Parameters
    /// * `method` - The HTTP method for this request.
    /// * `region_platform` - The stringified platform, used to create the base URL.
    /// * `path` - The URL path, appended to the base URL.
    pub fn request(&self, method: Method, region_platform: &str, path: &str) -> RequestBuilder {
        let base_url_platform = self.config.base_url.replace("{}", region_platform);
        self.client
            .request(method, format!("{}{}", base_url_platform, path))
    }

    /// This method should generally not be used directly. Consider using endpoint wrappers instead.
    ///
    /// This sends a request based on the given parameters and returns a parsed result.
    ///
    /// # Parameters
    /// * `method_id` - A unique string id representing the endpoint method for per-method rate limiting.
    /// * `region_platform` - The stringified platform, used in rate limiting.
    /// * `request` - The request information. Use `request()` to obtain a `RequestBuilder` instance.
    ///
    /// # Returns
    /// A future resolving to a `Result` containg either a `T` (success) or a `RiotApiError` (failure).
    pub async fn execute_val<'a, T: serde::de::DeserializeOwned + 'a>(
        &'a self,
        method_id: &'static str,
        region_platform: &'static str,
        request: RequestBuilder,
    ) -> Result<T> {
        let rinfo = self
            .execute_raw(method_id, region_platform, request)
            .await?;
        let retries = rinfo.retries;
        let status = rinfo.response.status();
        let value = rinfo.response.json::<T>().await;
        value.map_err(|e| RiotApiError::new(e, retries, None, Some(status)))
    }

    /// This method should generally not be used directly. Consider using endpoint wrappers instead.
    ///
    /// This sends a request based on the given parameters and returns an optional parsed result.
    ///
    /// # Parameters
    /// * `method_id` - A unique string id representing the endpoint method for per-method rate limiting.
    /// * `region_platform` - The stringified platform, used in rate limiting.
    /// * `request` - The request information. Use `request()` to obtain a `RequestBuilder` instance.
    ///
    /// # Returns
    /// A future resolving to a `Result` containg either an `Option<T>` (success) or a `RiotApiError` (failure).
    pub async fn execute_opt<'a, T: serde::de::DeserializeOwned + 'a>(
        &'a self,
        method_id: &'static str,
        region_platform: &'static str,
        request: RequestBuilder,
    ) -> Result<Option<T>> {
        let rinfo = self
            .execute_raw(method_id, region_platform, request)
            .await?;
        if rinfo.status_none {
            return Ok(None);
        }
        let retries = rinfo.retries;
        let status = rinfo.response.status();
        let value = rinfo.response.json::<Option<T>>().await;
        value.map_err(|e| RiotApiError::new(e, retries, None, Some(status)))
    }

    /// This method should generally not be used directly. Consider using endpoint wrappers instead.
    ///
    /// This sends a request based on the given parameters but does not deserialize any response body.
    ///
    /// # Parameters
    /// * `method_id` - A unique string id representing the endpoint method for per-method rate limiting.
    /// * `region_platform` - The stringified platform, used in rate limiting.
    /// * `request` - The request information. Use `request()` to obtain a `RequestBuilder` instance.
    ///
    /// # Returns
    /// A future resolving to a `Result` containg either `()` (success) or a `RiotApiError` (failure).
    pub async fn execute(
        &self,
        method_id: &'static str,
        region_platform: &'static str,
        request: RequestBuilder,
    ) -> Result<()> {
        let rinfo = self
            .execute_raw(method_id, region_platform, request)
            .await?;
        let retries = rinfo.retries;
        let status = rinfo.response.status();
        rinfo
            .response
            .error_for_status()
            .map(|_| ())
            .map_err(|e| RiotApiError::new(e, retries, None, Some(status)))
    }

    /// This method should generally not be used directly. Consider using endpoint wrappers instead.
    ///
    /// This sends a request based on the given parameters and returns a raw `ResponseInfo`.
    ///
    /// This can be used to implement a Riot API proxy without needing to deserialize and reserialize JSON responses.
    ///
    /// # Parameters
    /// * `method_id` - A unique string id representing the endpoint method for per-method rate limiting.
    /// * `region_platform` - The stringified platform, used in rate limiting.
    /// * `request` - The request information. Use `request()` to obtain a `RequestBuilder` instance.
    ///
    /// # Returns
    /// A future resolving to a `Result` containg either a `ResponseInfo` (success) or a `RiotApiError` (failure).
    pub fn execute_raw(
        &self,
        method_id: &'static str,
        region_platform: &'static str,
        request: RequestBuilder,
    ) -> impl Future<Output = Result<ResponseInfo>> + '_ {
        self.regional_requester(region_platform)
            .execute(&self.config, method_id, request)
    }

    /// Get or create the RegionalRequester for the given region.
    fn regional_requester(&self, region_platform: &'static str) -> Arc<RegionalRequester> {
        self.regional_requesters
            .get_or_insert_with(region_platform, || {
                log::debug!(
                    "Creating requester for region platform {}.",
                    region_platform
                );
                RegionalRequester::new()
            })
    }
}
