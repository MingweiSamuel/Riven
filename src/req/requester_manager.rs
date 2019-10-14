use std::collections::HashMap;

use reqwest::{
    Client,
};

use super::regional_requester::RegionalRequester;
use crate::riot_api_config::RiotApiConfig;
use crate::consts::region::Region;

// pub struct RequesterManager<'a> {
//     /// Configuration settings.
//     riot_api_config: &'a RiotApiConfig<'a>,
//     /// Client for making requests.
//     client: &'a Client,

//     /// Represents the app rate limit.
//     app_rate_limit: RateLimit,
//     /// Represents method rate limits.
//     method_rate_limits: HashMap<&'a str, RateLimit>,
// }