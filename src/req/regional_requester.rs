use std::future::Future;
use std::sync::Arc;

use log;
use reqwest::{ StatusCode, RequestBuilder };

use crate::Result;
use crate::ResponseInfo;
use crate::RiotApiError;
use crate::RiotApiConfig;
use crate::util::InsertOnlyCHashMap;

use super::RateLimit;
use super::RateLimitType;

pub struct RegionalRequester {
    /// The app rate limit.
    app_rate_limit: RateLimit,
    /// Method rate limits.
    method_rate_limits: InsertOnlyCHashMap<&'static str, RateLimit>,
}

impl RegionalRequester {
    /// HTTP status codes which are considered a success but will results in `None`.
    const NONE_STATUS_CODES: [StatusCode; 2] = [
        StatusCode::NO_CONTENT, // 204
        StatusCode::NOT_FOUND,  // 404
    ];

    pub fn new() -> Self {
        Self {
            app_rate_limit: RateLimit::new(RateLimitType::Application),
            method_rate_limits: InsertOnlyCHashMap::new(),
        }
    }

    pub fn execute<'a>(self: Arc<Self>,
        config: &'a RiotApiConfig,
        method_id: &'static str, request: RequestBuilder)
        -> impl Future<Output = Result<ResponseInfo>> + 'a
    {
        async move {
            let mut retries: u8 = 0;
            loop {
                let method_rate_limit: Arc<RateLimit> = self.method_rate_limits
                    .get_or_insert_with(method_id, || RateLimit::new(RateLimitType::Method));

                // Rate limiting.
                while let Some(delay) = RateLimit::get_both_or_delay(&self.app_rate_limit, &*method_rate_limit) {
                    tokio::time::sleep(delay).await;
                }

                // Send request.
                let request_clone = request.try_clone().expect("Failed to clone request.");
                let response = request_clone.send().await
                    .map_err(|e| RiotApiError::new(e, retries, None, None))?;

                // Maybe update rate limits (based on response headers).
                self.app_rate_limit.on_response(&config, &response);
                method_rate_limit.on_response(&config, &response);

                let status = response.status();
                // Handle normal success / failure cases.
                let status_none = Self::NONE_STATUS_CODES.contains(&status);
                // Success case.
                if status.is_success() || status_none {
                    log::trace!("Response {} (retried {} times), success, returning result.", status, retries);
                    break Ok(ResponseInfo {
                        response: response,
                        retries: retries,
                        status_none: status_none,
                    });
                }
                let err = response.error_for_status_ref().err().unwrap_or_else(
                    || panic!("Unhandlable response status code, neither success nor failure: {}.", status));
                // Failure, may or may not be retryable.
                // Not-retryable: no more retries or 4xx or ? (3xx, redirects exceeded).
                // Retryable: retries remaining, and 429 or 5xx.
                if retries >= config.retries ||
                    (StatusCode::TOO_MANY_REQUESTS != status
                    && !status.is_server_error())
                {
                    log::debug!("Response {} (retried {} times), failure, returning error.", status, retries);
                    break Err(RiotApiError::new(err, retries, Some(response), Some(status)));
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
