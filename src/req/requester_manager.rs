use std::collections::HashMap;
use std::sync::Arc;

use reqwest::{
    Client,
};

use super::regional_requester::RegionalRequester;
use crate::riot_api_config::RiotApiConfig;
use crate::consts::region::Region;

pub struct RequesterManager<'a> {
    /// Configuration settings.
    riot_api_config: &'a RiotApiConfig<'a>,
    /// Client for making requests.
    client: &'a Client,

    /// Per-region requesters.
    regional_requesters: HashMap<&'a Region<'a>, Arc<RegionalRequester<'a>>>,
}