use std::future::Future;

use log::*;
use reqwest::Client;

use crate::riot_api_config::RiotApiConfig;
use crate::consts::Region;
use crate::util::InsertOnlyCHashMap;

use super::RegionalRequester;

pub struct RequesterManager {
    /// Configuration settings.
    riot_api_config: RiotApiConfig,
    /// Client for making requests.
    client: Client,

    /// Per-region requesters.
    regional_requesters: InsertOnlyCHashMap<Region, RegionalRequester>,
}

impl RequesterManager {
    pub fn new(riot_api_config: RiotApiConfig) -> Self {
        // TODO configure client.
        let client = Client::new();
        trace!("Creating client (TODO: configuration).");

        Self {
            riot_api_config: riot_api_config,
            client: client,
            regional_requesters: InsertOnlyCHashMap::new(),
        }
    }

    pub fn get_regional_requester(&self, region: Region) {
        self.regional_requesters
            .get_or_insert_with(region, || RegionalRequester::new())
    }

    // pub fn get<T>(
    //     &self, method_id: &'static str, region: Region, path: &str,
    //     query: Option<&str>) -> dyn Future<Result<Option<T>, reqwest::Error>>
    // {
    //     // TODO: max concurrent requests? Or can configure client?
    //     let regional_requester = self.regional_requesters
    //         .get_or_insert_with(region, || RegionalRequester::new());
    //     regional_requester
    //         .get(&self.riot_api_config, &self.client, method_id, region, path, query)
    // }
}
