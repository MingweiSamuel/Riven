use std::future::Future;

use log;
use reqwest::Client;

use crate::Result;
use crate::RiotApiConfig;
use crate::req::RegionalRequester;
use crate::util::InsertOnlyCHashMap;

/// For retrieving data from the Riot Games API.
///
/// # Rate Limiting
///
/// The Riot Game API does _dynamic_ rate limiting, meaning that rate limits are
/// specified in response headers and (hypothetically) could change at any time.
/// Riven keeps track of changing rate limits seamlessly, preventing you from
/// getting blacklisted.
///
/// Riven's rate limiting is highly efficient, meaning that it can reach the limits
/// of your rate limit without going over.
pub struct RiotApi {
    /// Configuration settings.
    config: RiotApiConfig,
    /// Client for making requests.
    client: Client,

    /// Per-region requesters.
    regional_requesters: InsertOnlyCHashMap<&'static str, RegionalRequester>,
}

impl RiotApi {
    pub fn with_config(mut config: RiotApiConfig) -> Self {
        let client_builder = config.client_builder.take()
            .expect("!NONE CLIENT_BUILDER IN CONFIG.");
        Self {
            config: config,
            client: client_builder.build().expect("Failed to create client from builder."),
            regional_requesters: InsertOnlyCHashMap::new(),
        }
    }

    pub fn with_key<T: Into<String>>(api_key: T) -> Self {
        Self::with_config(RiotApiConfig::with_key(api_key))
    }

    pub fn get<'a, T: serde::de::DeserializeOwned + 'a>(&'a self,
        method_id: &'static str, region_platform: &'static str, path: String, query: Option<String>)
        -> impl Future<Output = Result<Option<T>>> + 'a
    {
        self.regional_requesters
            .get_or_insert_with(region_platform, || {
                log::debug!("Creating requester for region platform {}.", region_platform);
                RegionalRequester::new()
            })
            .get(&self.config, &self.client, method_id, region_platform, path, query)
    }
}
