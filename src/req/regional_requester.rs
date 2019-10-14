use std::collections::HashMap;
use std::future::Future;
use std::sync::RwLock;

use async_std::task;
use reqwest::{
    Client,
    StatusCode,
};
use serde::de::DeserializeOwned;

use super::rate_limit::RateLimit;
use super::rate_limit_type::RateLimitType;
use crate::riot_api_config::RiotApiConfig;
use crate::consts::region::Region;

pub struct RegionalRequester<'a> {
    /// Configuration settings.
    riot_api_config: &'a RiotApiConfig<'a>,
    /// Client for making requests.
    client: &'a Client,

    /// Represents the app rate limit.
    app_rate_limit: RateLimit,
    /// Represents method rate limits.
    method_rate_limits: HashMap<&'a str, RateLimit>,
}

impl <'a> RegionalRequester<'a> {
    /// Request header name for the Riot API key.
    const RIOT_KEY_HEADER: &'static str = "X-Riot-Token";

    /// HttpStatus codes that are considered a success, but will return None.
    const NONE_STATUS_CODES: [u16; 3] = [ 204, 404, 422 ];


    pub fn new(riot_api_config: &'a RiotApiConfig<'a>, client: &'a Client) -> RegionalRequester<'a> {
        RegionalRequester {
            riot_api_config: riot_api_config,
            client: client,
            app_rate_limit: RateLimit::new(RateLimitType::Application),
            method_rate_limits: HashMap::new(),
        }
    }

    pub async fn get<T: DeserializeOwned>(
        &mut self, method_id: &'a str, relative_url: &'_ str,
        region: &'_ Region<'_>, query: &[(&'_ str, &'_ str)]) -> Result<Option<T>, reqwest::Error> {

        let mut attempts: u8 = 0;
        for _ in 0..=self.riot_api_config.retries {
            attempts += 1;

            // Rate limiting.
            let app_rate_limit = &self.app_rate_limit;
            let method_rate_limit = self.method_rate_limits.entry(method_id)
                .or_insert_with(|| RateLimit::new(RateLimitType::Method));

            while let Some(delay) = RateLimit::get_both_or_delay(app_rate_limit, method_rate_limit) {
                task::sleep(delay).await;
            }

            // Send request.
            let url = &*format!("https://{}.api.riotgames.com{}", region.platform, relative_url);
            let result = self.client.get(url)
                .header(Self::RIOT_KEY_HEADER, self.riot_api_config.api_key)
                .query(query)
                .send()
                .await;

            let response = match result {
                Err(e) => return Err(e),
                Ok(r) => r,
            };

            // Update rate limits (if needed).
            app_rate_limit.on_response(&response);
            method_rate_limit.on_response(&response);

            // Handle response.
            let status = response.status();
            // Success, return None.
            if Self::is_none_status_code(&status) {
                return Ok(None);
            }
            // Success, return a value.
            if status.is_success() {
                let value = response.json::<T>().await;
                return match value {
                    Err(e) => Err(e),
                    Ok(v) => Ok(Some(v)),
                }
            }
            // Retryable.
            if StatusCode::TOO_MANY_REQUESTS == status || status.is_server_error() {
                continue;
            }
            // Failure (non-retryable).
            if status.is_client_error() {
                break;
            }
            panic!("NOT HANDLED: {}!", status);
        }
        // TODO: return error.
        panic!("FAILED AFTER {} ATTEMPTS!", attempts);
    }

    pub fn get2<T: 'a + DeserializeOwned>(&'a mut self, method_id: &'a str, relative_url: &'a str,
        region: &'a Region<'_>, query: &'a [(&'a str, &'a str)]) -> impl Future<Output = Result<Option<T>, reqwest::Error>> + 'a {

        self.get(method_id, relative_url, region, query)
    }

    fn is_none_status_code(status: &StatusCode) -> bool {
        Self::NONE_STATUS_CODES.contains(&status.as_u16())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn send_sync() {
        fn is_send_sync<T: Send + Sync>() {}
        is_send_sync::<RegionalRequester>();
    }
}
