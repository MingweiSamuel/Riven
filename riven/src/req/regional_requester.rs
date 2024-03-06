use std::future::Future;
use std::sync::Arc;

use reqwest::{RequestBuilder, StatusCode};
#[cfg(feature = "tracing")]
use tracing::{self as log, Instrument};

use super::{RateLimit, RateLimitType};
use crate::time::{sleep, Duration};
use crate::util::InsertOnlyCHashMap;
use crate::{ResponseInfo, Result, RiotApiConfig, RiotApiError};

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

    pub fn execute<'a>(
        self: Arc<Self>,
        config: &'a RiotApiConfig,
        method_id: &'static str,
        request: RequestBuilder,
    ) -> impl Future<Output = Result<ResponseInfo>> + 'a {
        async move {
            let mut retries: u8 = 0;
            loop {
                let method_rate_limit: Arc<RateLimit> = self
                    .method_rate_limits
                    .get_or_insert_with(method_id, || RateLimit::new(RateLimitType::Method));

                // Rate limit.
                let rate_limit = RateLimit::acquire_both(&self.app_rate_limit, &method_rate_limit);
                #[cfg(feature = "tracing")]
                let rate_limit = rate_limit.instrument(tracing::info_span!("rate_limit"));
                rate_limit.await;

                // Send request.
                let request_clone = request
                    .try_clone()
                    .expect("Failed to clone request.")
                    .send();
                #[cfg(feature = "tracing")]
                let request_clone = request_clone.instrument(tracing::info_span!("request"));
                let response = request_clone.await;
                let response = match response {
                    Ok(response) => response,
                    // Check for lower level errors, like connection errors.
                    Err(e) => {
                        if retries >= config.retries {
                            log::debug!(
                                "Request failed (retried {} times), failure, returning error.",
                                retries
                            );
                            break Err(RiotApiError::new(e, retries, None, None));
                        }
                        let delay = Duration::from_secs(2_u64.pow(retries as u32));
                        log::debug!("Request failed with cause \"{}\", (retried {} times), using exponential backoff, retrying after {:?}.", e.to_string(), retries, delay);
                        let backoff = sleep(delay);
                        #[cfg(feature = "tracing")]
                        let backoff = backoff.instrument(tracing::info_span!("backoff"));
                        backoff.await;
                        retries += 1;
                        continue;
                    }
                };
                // Maybe update rate limits (based on response headers).
                // Use single bar for no short circuiting.
                let retry_after_app = self.app_rate_limit.on_response(config, &response);
                let retry_after_method = method_rate_limit.on_response(config, &response);
                let retry_after = retry_after_app.or(retry_after_method); // Note: Edge case if both are Some(_) not handled.

                let status = response.status();
                // Handle normal success / failure cases.
                let status_none = Self::NONE_STATUS_CODES.contains(&status);
                // Success case.
                if status.is_success() || status_none {
                    log::trace!(
                        "Response {} (retried {} times), success, returning result.",
                        status,
                        retries
                    );
                    break Ok(ResponseInfo {
                        response,
                        retries,
                        status_none,
                    });
                }
                let err = response.error_for_status_ref().err().unwrap_or_else(|| {
                    panic!(
                        "Unhandlable response status code, neither success nor failure: {}.",
                        status
                    )
                });
                // Failure, may or may not be retryable.
                // Not-retryable: no more retries or 4xx or ? (3xx, redirects exceeded).
                // Retryable: retries remaining, and 429 or 5xx.
                if retries >= config.retries
                    || (StatusCode::TOO_MANY_REQUESTS != status && !status.is_server_error())
                {
                    log::debug!(
                        "Response {} (retried {} times), failure, returning error.",
                        status,
                        retries
                    );
                    break Err(RiotApiError::new(
                        err,
                        retries,
                        Some(response),
                        Some(status),
                    ));
                }

                // Is retryable, do exponential backoff if retry-after wasn't specified.
                // 1 sec, 2 sec, 4 sec, 8 sec.
                match retry_after {
                    None => {
                        let delay = Duration::from_secs(2_u64.pow(retries as u32));
                        log::debug!("Response {} (retried {} times), NO `retry-after`, using exponential backoff, retrying after {:?}.", status, retries, delay);
                        let backoff = sleep(delay);
                        #[cfg(feature = "tracing")]
                        let backoff = backoff.instrument(tracing::info_span!("backoff"));
                        backoff.await;
                    }
                    Some(delay) => {
                        log::debug!("Response {} (retried {} times), `retry-after` set, retrying after {:?}.", status, retries, delay);
                    }
                }
                retries += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
