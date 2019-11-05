use std::future::Future;
use std::sync::Arc;

use log;
use reqwest::{ Client, StatusCode, Url };
use tokio::timer::delay_for;

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

    /// HttpStatus codes that are considered a success, but will return None.
    const NONE_STATUS_CODES: [u16; 3] = [ 204, 404, 422 ];


    pub fn new() -> Self {
        Self {
            app_rate_limit: RateLimit::new(RateLimitType::Application),
            method_rate_limits: InsertOnlyCHashMap::new(),
        }
    }

    pub fn get<'a, T: serde::de::DeserializeOwned>(self: Arc<Self>,
        config: &'a RiotApiConfig, client: &'a Client,
        method_id: &'static str, region_platform: &'a str, path: String, query: Option<String>)
        -> impl Future<Output = Result<Option<T>>> + 'a
    {
        async move {
            let query = query.as_deref();

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

                // Handle response.
                let status = response.status();
                // Special "none success" cases, return None.
                if Self::is_none_status_code(&status) {
                    log::trace!("Response {} (retried {} times), None result.", status, retries);
                    break Ok(None);
                }
                // Handle normal success / failure cases.
                match response.error_for_status_ref() {
                    // Success.
                    Ok(_) => {
                        log::trace!("Response {} (retried {} times), parsed result.", status, retries);
                        let value = response.json::<T>().await;
                        break value.map(|v| Some(v))
                            .map_err(|e| RiotApiError::new(e, retries, None));
                    },
                    // Failure, may or may not be retryable.
                    Err(err) => {
                        // Not-retryable: no more retries or 4xx or ? (3xx, redirects exceeded).
                        // Retryable: retries remaining, and 429 or 5xx.
                        if retries >= config.retries ||
                            (StatusCode::TOO_MANY_REQUESTS != status
                            && !status.is_server_error())
                        {
                            log::debug!("Response {} (retried {} times), returning error.", status, retries);
                            break Err(RiotApiError::new(err, retries, Some(response)));
                        }
                        log::debug!("Response {} (retried {} times), retrying.", status, retries);
                    },
                };

                retries += 1;
            }
        }
    }

    fn is_none_status_code(status: &StatusCode) -> bool {
        Self::NONE_STATUS_CODES.contains(&status.as_u16())
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
