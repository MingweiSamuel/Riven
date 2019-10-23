use std::future::Future;

use log;
use reqwest::Client;

use crate::RiotApiConfig;
use crate::consts::Region;
use crate::req::RegionalRequester;
use crate::util::InsertOnlyCHashMap;

pub struct RiotApi {
    /// Configuration settings.
    config: RiotApiConfig,
    /// Client for making requests.
    client: Client,

    /// Per-region requesters.
    regional_requesters: InsertOnlyCHashMap<Region, RegionalRequester>,
}

impl RiotApi {
    pub fn with_config(config: RiotApiConfig) -> Self {
        log::trace!("Creating client (TODO: configuration).");
        Self {
            config: config,
            client: Client::new(),
            regional_requesters: InsertOnlyCHashMap::new(),
        }
    }

    pub fn with_key<T: Into<String>>(api_key: T) -> Self {
        Self::with_config(RiotApiConfig::with_key(api_key))
    }

    pub fn get<'a, T: serde::de::DeserializeOwned + 'a>(&'a self,
        method_id: &'static str, region: Region, path: String, query: Option<String>)
        -> impl Future<Output = Result<Option<T>, reqwest::Error>> + 'a
    {
        // TODO: max concurrent requests? Or can configure client?
        self.regional_requesters
            .get_or_insert_with(region, || RegionalRequester::new())
            .get(&self.config, &self.client, method_id, region, path, query)
    }
}
