use std::future::Future;
use std::sync::Arc;

use async_std::task;
use reqwest::{
    Client,
    StatusCode,
};
use parking_lot::Mutex;

use crate::riot_api_config::RiotApiConfig;
use crate::consts::Region;
use crate::util::InsertOnlyCHashMap;

use super::RateLimit;
use super::RateLimitType;

pub struct RegionalRequester<'a> {
    /// Configuration settings.
    riot_api_config: &'a RiotApiConfig<'a>,
    /// Client for making requests.
    client: &'a Client,

    /// Represents the app rate limit.
    app_rate_limit: RateLimit,
    /// Represents method rate limits.
    method_rate_limits: InsertOnlyCHashMap<&'a str, RateLimit>,
}

impl<'a> RegionalRequester<'a> {
    /// Request header name for the Riot API key.
    const RIOT_KEY_HEADER: &'static str = "X-Riot-Token";

    /// HttpStatus codes that are considered a success, but will return None.
    const NONE_STATUS_CODES: [u16; 3] = [ 204, 404, 422 ];


    pub fn new(riot_api_config: &'a RiotApiConfig<'a>, client: &'a Client) -> Self {
        Self {
            riot_api_config: riot_api_config,
            client: client,
            app_rate_limit: RateLimit::new(RateLimitType::Application),
            method_rate_limits: InsertOnlyCHashMap::new(),
        }
    }

    pub async fn get<T: serde::de::DeserializeOwned>(
        &self, method_id: &'a str, region: &'_ Region<'_>, relative_url: &'_ str,
        query: &[(&'_ str, &'_ str)]) -> Result<Option<T>, reqwest::Error>
    {

        let mut attempts: u8 = 0;
        for _ in 0..=self.riot_api_config.retries {
            attempts += 1;

            let method_rate_limit: Arc<RateLimit> = self.method_rate_limits
                .get_or_insert_with(method_id, || RateLimit::new(RateLimitType::Method));

            // Rate limiting.
            while let Some(delay) = RateLimit::get_both_or_delay(&self.app_rate_limit, &*method_rate_limit) {
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

            // Maybe update rate limits (based on response headers).
            self.app_rate_limit.on_response(&response);
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

    fn is_none_status_code(status: &StatusCode) -> bool {
        Self::NONE_STATUS_CODES.contains(&status.as_u16())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
