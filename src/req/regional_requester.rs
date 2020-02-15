use std::future::Future;
use std::sync::Arc;

use reqwest::{ Client, StatusCode, Url };
use tokio::time::delay_for;

#[cfg(feature = "trace")]
use tracing::{debug, trace};

#[cfg(not(feature = "trace"))]
use log::{debug, trace};

use crate::Result;
use crate::RiotApiError;
use crate::RiotApiConfig;
use crate::util::InsertOnlyCHashMap;

use super::RateLimit;
use super::RateLimitType;

pub struct RegionalRequester {
    /// Represents the app rate limit.
    app_rate_limit: RateLimit,
    /// Represents method rate limits.
    method_rate_limits: InsertOnlyCHashMap<&'static str, RateLimit>,
}

impl RegionalRequester {
    /// Request header name for the Riot API key.
    const RIOT_KEY_HEADER: &'static str = "X-Riot-Token";

    /// Http status code 404, considered a success but will return None.
    const NONE_STATUS_CODE: StatusCode = StatusCode::NOT_FOUND;


    pub fn new() -> Self {
        Self {
            app_rate_limit: RateLimit::new(RateLimitType::Application),
            method_rate_limits: InsertOnlyCHashMap::new(),
        }
    }

    pub fn get_optional<'a, T: serde::de::DeserializeOwned>(self: Arc<Self>,
        config: &'a RiotApiConfig, client: &'a Client,
        method_id: &'static str, region_platform: &'a str, path: String, query: Option<String>)
        -> impl Future<Output = Result<Option<T>>> + 'a
    {
        async move {
            let response_result = self.get(config, client,
                method_id, region_platform, path, query).await;
            response_result.map(|value| Some(value))
                .or_else(|e| {
                    if let Some(response) = e.response() {
                    if Self::NONE_STATUS_CODE == response.status() {
                        return Ok(None);
                    }}
                    Err(e)
                })
        }
    }

    pub fn get<'a, T: serde::de::DeserializeOwned>(self: Arc<Self>,
        config: &'a RiotApiConfig, client: &'a Client,
        method_id: &'static str, region_platform: &'a str, path: String, query: Option<String>)
        -> impl Future<Output = Result<T>> + 'a
    {
        async move {
            #[cfg(feature = "nightly")] let query = query.as_deref();
            #[cfg(not(feature = "nightly"))] let query = query.as_ref().map(|s| s.as_ref());

            let mut retries: u8 = 0;
            loop {
                let method_rate_limit: Arc<RateLimit> = self.method_rate_limits
                    .get_or_insert_with(method_id, || RateLimit::new(RateLimitType::Method));

                // Rate limiting.
                while let Some(delay) = RateLimit::get_both_or_delay(&self.app_rate_limit, &*method_rate_limit) {
                    delay_for(delay).await;
                }

                // Send request.
                let url_base = format!("https://{}.api.riotgames.com", region_platform);
                let mut url = Url::parse(&*url_base)
                    .unwrap_or_else(|_| panic!("Failed to parse url_base: \"{}\".", url_base));
                url.set_path(&*path);
                url.set_query(query);

                let response = client.get(url)
                    .header(Self::RIOT_KEY_HEADER, &*config.api_key)
                    .send()
                    .await
                    .map_err(|e| RiotApiError::new(e, retries, None))?;

                // Maybe update rate limits (based on response headers).
                self.app_rate_limit.on_response(&config, &response);
                method_rate_limit.on_response(&config, &response);

                let status = response.status();
                // Handle normal success / failure cases.
                match response.error_for_status_ref() {
                    // Success.
                    Ok(_response) => {
                        trace!("Response {} (retried {} times), parsed result.", status, retries);
                        let value = response.json::<T>().await;
                        break value.map_err(|e| RiotApiError::new(e, retries, None));
                    },
                    // Failure, may or may not be retryable.
                    Err(err) => {
                        // Not-retryable: no more retries or 4xx or ? (3xx, redirects exceeded).
                        // Retryable: retries remaining, and 429 or 5xx.
                        if retries >= config.retries ||
                            (StatusCode::TOO_MANY_REQUESTS != status
                            && !status.is_server_error())
                        {
                            debug!("Response {} (retried {} times), returning.", status, retries);
                            break Err(RiotApiError::new(err, retries, Some(response)));
                        }
                        debug!("Response {} (retried {} times), retrying.", status, retries);
                    },
                };

                retries += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
