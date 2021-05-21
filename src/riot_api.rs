use std::future::Future;
use std::sync::Arc;

use log;
use reqwest::{Client, Request, Method, Url};

use crate::Result;
use crate::ResponseInfo;
use crate::RiotApiConfig;
use crate::RiotApiError;
use crate::req::RegionalRequester;
use crate::util::InsertOnlyCHashMap;

/// For retrieving data from the Riot Games API.
///
/// # Usage
///
/// Construct an instance using [`with_key(api_key)`](RiotApi::with_key) or
/// [`with_config(config)`](RiotApi::with_config).
///
/// An instance provides access to "endpoint handles" which in turn provide access
/// to individual API method calls. For example, getting a summoner by name:
/// ```ignore
/// riot_api.summoner_v4().get_by_summoner_name(Region::NA, "LugnutsK")
/// ```
///
/// # Rate Limiting
///
/// The Riot Game API enforces _dynamic_ rate limiting, meaning that rate limits are
/// specified in response headers and (theoretically) could change at any time.
/// Riven keeps track of changing rate limits seamlessly, preventing you from
/// getting blacklisted.
///
/// Riven's rate limiting is highly efficient, meaning that it can reach the limits
/// of your rate limit without going over.
///
/// To adjust rate limiting, see [RiotApiConfig](crate::RiotApiConfig) and use
/// [`with_config(config)`](RiotApi::with_config) to construct an instance.
pub struct RiotApi {
    /// Configuration settings.
    config: RiotApiConfig,
    /// Client for making requests.
    client: Client,

    /// Per-region requesters.
    regional_requesters: InsertOnlyCHashMap<&'static str, RegionalRequester>,
}

impl RiotApi {
    /// Constructs a new instance from the given [RiotApiConfig](crate::RiotApiConfig), consuming it.
    pub fn with_config(mut config: RiotApiConfig) -> Self {
        let client_builder = config.client_builder.take()
            .expect("!NONE CLIENT_BUILDER IN CONFIG.");
        Self {
            config: config,
            client: client_builder.build().expect("Failed to create client from builder."),
            regional_requesters: InsertOnlyCHashMap::new(),
        }
    }

    /// Constructs a new instance from the given API key, using default configuration.
    ///
    /// `api_key` should be a Riot Games API key from
    /// [https://developer.riotgames.com/](https://developer.riotgames.com/),
    /// and should look like `"RGAPI-01234567-89ab-cdef-0123-456789abcdef"`.
    pub fn with_key<T: AsRef<[u8]>>(api_key: T) -> Self {
        Self::with_config(RiotApiConfig::with_key(api_key))
    }

    /// This method is not meant to be used directly.
    ///
    /// This sends a GET request based on the given parameters and returns an optional parsed result.
    ///
    /// # Parameters
    /// * `method_id` - A unique string id representing the endpoint method for per-method rate limiting.
    /// * `region_platform` - The stringified platform, prepended to `.api.riotgames.com` to create the hostname.
    /// * `path` - The path relative to the hostname.
    /// * `query` - An optional query string.
    ///
    /// # Returns
    /// A future resolving to a `Result` containg either a `Option<T>` (success) or a `RiotApiError` (failure).
    pub async fn get_optional<'a, T: serde::de::DeserializeOwned + 'a>(&'a self,
        method_id: &'static str, region_platform: &'static str, path: String, query: Option<String>)
        -> Result<Option<T>>
    {
        let rinfo = self.get_raw_response(method_id, region_platform, path, query).await?;
        if rinfo.status_none {
            return Ok(None);
        }
        let retries = rinfo.retries;
        let status = rinfo.response.status();
        let value = rinfo.response.json::<Option<T>>().await;
        value.map_err(|e| RiotApiError::new(e, retries, None, Some(status)))
    }

    /// This method is not meant to be used directly.
    ///
    /// This sends a GET request based on the given parameters and returns a parsed result.
    ///
    /// # Parameters
    /// * `method_id` - A unique string id representing the endpoint method for per-method rate limiting.
    /// * `region_platform` - The stringified platform, prepended to `.api.riotgames.com` to create the hostname.
    /// * `path` - The path relative to the hostname.
    /// * `query` - An optional query string.
    ///
    /// # Returns
    /// A future resolving to a `Result` containg either a `T` (success) or a `RiotApiError` (failure).
    pub async fn get<'a, T: serde::de::DeserializeOwned + 'a>(&'a self,
        method_id: &'static str, region_platform: &'static str, path: String, query: Option<String>)
        -> Result<T>
    {
        let rinfo = self.get_raw_response(method_id, region_platform, path, query).await?;
        let retries = rinfo.retries;
        let status = rinfo.response.status();
        let value = rinfo.response.json::<T>().await;
        value.map_err(|e| RiotApiError::new(e, retries, None, Some(status)))
    }

    /// This method is not meant to be used directly.
    ///
    /// This sends a GET request based on the given parameters and returns a raw `ResponseInfo`.
    ///
    /// This can be used to implement a Riot API proxy without needing to deserialize and reserialize JSON responses.
    ///
    /// # Parameters
    /// * `method_id` - A unique string id representing the endpoint method for per-method rate limiting.
    /// * `region_platform` - The stringified platform, prepended to `.api.riotgames.com` to create the hostname.
    /// * `path` - The path relative to the hostname.
    /// * `query` - An optional query string.
    ///
    /// # Returns
    /// A future resolving to a `Result` containg either a `ResponseInfo` (success) or a `RiotApiError` (failure).
    pub fn get_raw_response(&self,
        method_id: &'static str, region_platform: &'static str, path: String, query: Option<String>)
        -> impl Future<Output = Result<ResponseInfo>> + '_
    {
        let url_base = format!("https://{}.api.riotgames.com", region_platform);
        let mut url = Url::parse(&*url_base)
            .unwrap_or_else(|_| panic!("Failed to parse url_base: \"{}\".", url_base));
        url.set_path(&*path);
        url.set_query(query.as_deref());

        let request = Request::new(Method::GET, url);
        self.execute_raw(method_id, region_platform, request)
    }





    pub async fn execute_optional<'a, T: serde::de::DeserializeOwned + 'a>(&'a self,
        method_id: &'static str, region_platform: &'static str, request: Request)
        -> Result<Option<T>>
    {
        let rinfo = self.execute_raw(method_id, region_platform, request).await?;
        if rinfo.status_none {
            return Ok(None);
        }
        let retries = rinfo.retries;
        let status = rinfo.response.status();
        let value = rinfo.response.json::<Option<T>>().await;
        value.map_err(|e| RiotApiError::new(e, retries, None, Some(status)))
    }

    pub async fn execute<'a, T: serde::de::DeserializeOwned + 'a>(&'a self,
        method_id: &'static str, region_platform: &'static str, request: Request)
        -> Result<T>
    {
        let rinfo = self.execute_raw(method_id, region_platform, request).await?;
        let retries = rinfo.retries;
        let status = rinfo.response.status();
        let value = rinfo.response.json::<T>().await;
        value.map_err(|e| RiotApiError::new(e, retries, None, Some(status)))
    }

    pub fn execute_raw(&self, method_id: &'static str, region_platform: &'static str, request: Request)
        -> impl Future<Output = Result<ResponseInfo>> + '_
    {
        self.regional_requester(region_platform)
            .execute_raw(&self.config, &self.client, method_id, request)
    }

    /// Get or create the RegionalRequester for the given region.
    fn regional_requester(&self, region_platform: &'static str) -> Arc<RegionalRequester> {
        self.regional_requesters.get_or_insert_with(region_platform, || {
            log::debug!("Creating requester for region platform {}.", region_platform);
            RegionalRequester::new()
        })
    }
}
