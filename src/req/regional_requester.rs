use std::future::Future;
use std::sync::Arc;

use log;
use tokio::time::delay_for;

use crate::client::{ Client, Response };
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

    /// HTTP status codes which are considered a success but will results in `None`.
    const NONE_STATUS_CODES: [u16; 2] = [
        204, // No Content.
        404, // Not Found.
    ];

    pub fn new() -> Self {
        Self {
            app_rate_limit: RateLimit::new(RateLimitType::Application),
            method_rate_limits: InsertOnlyCHashMap::new(),
        }
    }

    pub fn get_optional<'a, C: Client, T: serde::de::DeserializeOwned + 'static>(self: Arc<Self>,
        config: &'a RiotApiConfig, client: &'a C,
        method_id: &'static str, region_platform: &'a str, path: String, query: Option<String>)
        -> impl Future<Output = Result<Option<T>>> + 'a
    {
        async move {
            panic!("FIXME");
            // let response_result = self.get(config, client,
            //     method_id, region_platform, path, query).await;
            // response_result.map(|value| Some(value))
            //     .or_else(|e| {
            //         if let Some(status) = e.status_code() {
            //         if Self::NONE_STATUS_CODES.contains(&status) {
            //             return Ok(None);
            //         }}
            //         Err(e)
            //     })
        }
    }

    pub fn get<'a, C: Client, T: serde::de::DeserializeOwned + 'static>(self: Arc<Self>,
        config: &'a RiotApiConfig, client: &'a C,
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
                let response = client.get(url_base, &path, query, vec![( Self::RIOT_KEY_HEADER, &config.api_key )])
                    .await
                    .unwrap(); // FIXME
                    // .map_err(|e| RiotApiError::new(e, retries, None, None))?;

                // Maybe update rate limits (based on response headers).
                self.app_rate_limit.on_response(&config, &response);
                method_rate_limit.on_response(&config, &response);

                let status = response.status();
                // Handle normal success / failure cases.
                if 200 == status {
                    // Success.
                    log::trace!("Response {} (retried {} times), parsed result.", status, retries);
                    let value = response.into_json::<T>().await;
                    break value.map_err(|e| panic!("FIXME")); //RiotApiError::new(Some(e), retries, None, Some(status)));
                }
                // Not-retryable: no more retries or 4xx or ? (3xx, redirects exceeded).
                // Retryable: retries remaining, and 429 or 5xx.
                if retries >= config.retries || (429 != status && 500 > status)
                {
                    log::debug!("Response {} (retried {} times), returning.", status, retries);
                    panic!("FIXME"); // FIXME break Err(RiotApiError::new(None, retries, Some(response), Some(status)));
                }
                log::debug!("Response {} (retried {} times), retrying.", status, retries);

                retries += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
