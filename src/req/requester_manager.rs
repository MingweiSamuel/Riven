use log::*;
use reqwest::Client;

use crate::riot_api_config::RiotApiConfig;
use crate::consts::Region;
use crate::util::InsertOnlyCHashMap;

use super::RegionalRequester;

pub struct RequesterManager<'a> {
    /// Configuration settings.
    riot_api_config: RiotApiConfig<'a>,
    /// Client for making requests.
    client: Client,

    /// Per-region requesters.
    regional_requesters: InsertOnlyCHashMap<Region, RegionalRequester<'a>>,
}

impl<'a> RequesterManager<'a> {
    pub fn new(riot_api_config: RiotApiConfig<'a>) -> Self {
        // TODO configure client.
        let client = Client::new();
        trace!("Creating client (TODO: configuration).");

        Self {
            riot_api_config: riot_api_config,
            client: client,
            regional_requesters: InsertOnlyCHashMap::new(),
        }
    }

    pub async fn get<T: serde::de::DeserializeOwned>(
        &'a self, method_id: &'a str, region: Region, path: &str,
        query: Option<&str>) -> Result<Option<T>, reqwest::Error>
    {
        // TODO: max concurrent requests? Or can configure client?
        let regional_requester = self.regional_requesters
            .get_or_insert_with(region, || RegionalRequester::new(&self.riot_api_config, &self.client));
        regional_requester.get(method_id, region, path, query).await
    }
}
