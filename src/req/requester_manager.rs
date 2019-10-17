use std::collections::HashMap;
use std::sync::Arc;

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
    regional_requesters: InsertOnlyCHashMap<&'a Region<'a>, RegionalRequester<'a>>,
}

impl<'a> RequesterManager<'a> {
    pub fn new(riot_api_config: RiotApiConfig<'a>) -> Self {
        // TODO client.
        let client = Client::new();
        Self {
            riot_api_config: riot_api_config,
            client: client,
            regional_requesters: InsertOnlyCHashMap::new(),
        }
    }

    pub async fn get<T: serde::de::DeserializeOwned>(
        &'a self, method_id: &'a str, region: &'a Region<'a>, relative_url: &'_ str,
        query: &[(&'_ str, &'_ str)]) -> Result<Option<T>, reqwest::Error>
    {
        // TODO: max concurrent requests?
        let regional_requester = self.regional_requesters
            .get_or_insert_with(region, || RegionalRequester::new(&self.riot_api_config, &self.client));
        regional_requester.get(method_id, region, relative_url, query).await
    }
}
