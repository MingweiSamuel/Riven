use std::cmp;

use futures::FutureExt;
use parking_lot::{RwLock, RwLockUpgradableReadGuard};
use reqwest::{Response, StatusCode};
#[cfg(feature = "tracing")]
use tracing as log;

use super::{RateLimitType, TokenBucket, VectorTokenBucket};
use crate::time::{sleep, Duration, Instant};
use crate::util::Notify;
use crate::RiotApiConfig;

pub struct RateLimit {
    rate_limit_type: RateLimitType,
    // Buckets for this rate limit (synchronized).
    // Almost always read, written only when rate limit rates are updated
    // from API response.
    buckets: RwLock<Vec<VectorTokenBucket>>,
    // Set to when we can retry if a retry-after header is received.
    retry_after: RwLock<Option<Instant>>,
    // Notifies waiters when rate limits are updated.
    update_notify: Notify,
}

impl RateLimit {
    /// Header specifying which RateLimitType caused a 429.
    /// This header specifies which rate limit is violated in a 429 (if any).
    /// There are three possible values, see [HEADER_XRATELIMITTYPE_APPLICATION],
    /// [HEADER_XRATELIMITTYPE_METHOD], and [HEADER_XRATELIMITTYPE_SERVICE].
    const HEADER_XRATELIMITTYPE: &'static str = "X-Rate-Limit-Type";

    /// `"application"` - Entire app/key is rate limited due to violation.
    const HEADER_XRATELIMITTYPE_APPLICATION: &'static str = "application";
    /// `"method"` - App/key is rate limited on a specific method due to violation.
    const HEADER_XRATELIMITTYPE_METHOD: &'static str = "method";
    /// `"service"` - Service backend is rate-limiting (no violation).
    const HEADER_XRATELIMITTYPE_SERVICE: &'static str = "service";

    pub fn new(rate_limit_type: RateLimitType) -> Self {
        let initial_bucket =
            VectorTokenBucket::new(Duration::from_secs(1), 1, Duration::new(0, 0), 1.0, 1.0);
        RateLimit {
            rate_limit_type,
            // Rate limit before getting from response: 1/s.
            buckets: RwLock::new(vec![initial_bucket]),
            retry_after: RwLock::new(None),
            update_notify: Notify::new(),
        }
    }

    pub async fn acquire_both(app_rate_limit: &Self, method_rate_limit: &Self) {
        while let Some(delay) = Self::acquire_both_or_duration(app_rate_limit, method_rate_limit) {
            futures::select_biased! {
                _ = sleep(delay).fuse() => continue,
                _ = method_rate_limit.update_notify.notified() => {}
                _ = app_rate_limit.update_notify.notified() => {}
            };
            log::trace!("Task awoken due to rate limit update.");
        }
    }

    fn acquire_both_or_duration(
        app_rate_limit: &Self,
        method_rate_limit: &Self,
    ) -> Option<Duration> {
        // Check retry after.
        {
            let retry_after_delay = app_rate_limit.get_retry_after_delay().and_then(|a| {
                method_rate_limit
                    .get_retry_after_delay()
                    .map(|m| cmp::max(a, m))
            });
            if retry_after_delay.is_some() {
                return retry_after_delay;
            }
        }
        // Check buckets.
        let app_buckets = app_rate_limit.buckets.read();
        let method_buckets = method_rate_limit.buckets.read();
        for bucket in app_buckets.iter().chain(method_buckets.iter()) {
            let delay = bucket.get_delay();
            if delay.is_some() {
                return delay;
            }
        }
        // Success.
        for bucket in app_buckets.iter().chain(method_buckets.iter()) {
            bucket.get_tokens(1);
        }

        log::trace!(
            "Tokens obtained, buckets: APP {:?} METHOD {:?}",
            app_buckets,
            method_buckets
        );
        None
    }

    pub fn get_retry_after_delay(&self) -> Option<Duration> {
        self.retry_after
            .read()
            .and_then(|i| Instant::now().checked_duration_since(i))
    }

    /// Update retry-after and rate limits based on an API response.
    /// Returns the retry-after delay if set.
    pub fn on_response(&self, config: &RiotApiConfig, response: &Response) -> Option<Duration> {
        let retry_after = self.on_response_retry_after(response);
        self.on_response_rate_limits(config, response);
        retry_after
    }

    /// `on_response` helper for retry after check.
    /// Returns the retry-after delay if set.
    #[inline]
    fn on_response_retry_after(&self, response: &Response) -> Option<Duration> {
        // Only care about 429s.
        if StatusCode::TOO_MANY_REQUESTS != response.status() {
            return None;
        }

        {
            // Get the X-Rate-Limit-Type header, `Some("application" | "method" | "service")` or `None`.
            let header_opt = response.headers()
                .get(Self::HEADER_XRATELIMITTYPE)
                .or_else(|| {
                    log::info!("429 response missing {} header.", Self::HEADER_XRATELIMITTYPE);
                    None
                })
                .and_then(|header_value| header_value.to_str()
                    .map_err(|e| log::info!("429 response, error parsing '{}' header as string: {}. Header value: {:#?}",
                    Self::HEADER_XRATELIMITTYPE, e, header_value))
                    .ok());

            // This match checks the valid possibilities. Otherwise returns none to ignore.
            // `Application` handles "application", `Method` handles all other values.
            let is_responsible = match header_opt {
                Some(Self::HEADER_XRATELIMITTYPE_APPLICATION) => {
                    self.rate_limit_type == RateLimitType::Application
                }
                Some(Self::HEADER_XRATELIMITTYPE_METHOD | Self::HEADER_XRATELIMITTYPE_SERVICE) => {
                    self.rate_limit_type == RateLimitType::Method
                }
                other => {
                    // RateLimitType::Method handles unknown values.
                    if self.rate_limit_type == RateLimitType::Method {
                        log::warn!(
                            "429 response has None (missing or invalid) or unknown {} header value {:?}, {:?} rate limit obeying retry-after.",
                            Self::HEADER_XRATELIMITTYPE, other, self.rate_limit_type);
                        true
                    } else {
                        false
                    }
                }
            };
            if !is_responsible {
                return None;
            }
        }

        // Get retry after header. Only care if it exists.
        let retry_after_header = response
            .headers()
            .get(reqwest::header::RETRY_AFTER)
            .and_then(|h| {
                h.to_str()
                    .map_err(|e| {
                        log::error!(
                            "Failed to read retry-after header as visible ASCII string: {:?}.",
                            e
                        )
                    })
                    .ok()
            })?;

        log::info!(
            "429 response, rate limit {:?}, retry-after {} secs.",
            self.rate_limit_type,
            retry_after_header
        );

        // Header currently only returns ints, but float is more general. Can be zero.
        let retry_after_secs = retry_after_header
            .parse::<f32>()
            .map_err(|e| log::error!("Failed to parse read-after header as f32: {:?}.", e))
            .ok()?;

        // Add 0.5 seconds to account for rounding, cases when response is zero.
        let delay = Duration::from_secs_f32(0.5 + retry_after_secs);

        // Set `self.retry_after`.
        *self.retry_after.write() = Some(Instant::now() + delay);
        Some(delay)
    }

    #[inline]
    fn on_response_rate_limits(&self, config: &RiotApiConfig, response: &Response) {
        // Check if rate limits changed.
        let headers = response.headers();
        let limit_header_opt = headers
            .get(self.rate_limit_type.limit_header())
            .and_then(|h| {
                h.to_str()
                    .map_err(|e| {
                        log::error!(
                            "Failed to read limit header as visible ASCII string: {:?}.",
                            e
                        )
                    })
                    .ok()
            });
        let count_header_opt = headers
            .get(self.rate_limit_type.count_header())
            .and_then(|h| {
                h.to_str()
                    .map_err(|e| {
                        log::error!(
                            "Failed to read count header as visible ASCII string: {:?}.",
                            e
                        )
                    })
                    .ok()
            });

        if let (Some(limit_header), Some(count_header)) = (limit_header_opt, count_header_opt) {
            {
                let buckets = self.buckets.upgradable_read();
                if !buckets_require_updating(limit_header, &buckets) {
                    return;
                }

                // Buckets require updating. Upgrade to write lock.
                let mut buckets = RwLockUpgradableReadGuard::upgrade(buckets);
                *buckets =
                    buckets_from_header(config, limit_header, count_header, self.rate_limit_type);
            }
            // Notify waiters that buckets have updated (after unlocking).
            self.update_notify.notify_waiters();
        }
    }
}

fn buckets_require_updating(limit_header: &str, buckets: &[impl TokenBucket]) -> bool {
    if buckets.len() != limit_header.split(',').count() {
        return true;
    }
    for (limit_header_entry, bucket) in limit_header.split(',').zip(buckets) {
        // limit_header_entry "100:60" means 100 req per 60 sec.
        let bucket_entry = format!(
            "{}:{}",
            bucket.get_total_limit(),
            bucket.get_bucket_duration().as_secs()
        );
        if limit_header_entry != bucket_entry {
            return true;
        }
    }
    false
}

fn buckets_from_header(
    config: &RiotApiConfig,
    limit_header: &str,
    count_header: &str,
    rate_limit_type: RateLimitType,
) -> Vec<VectorTokenBucket> {
    // Limits: "20000:10,1200000:600"
    // Counts: "7:10,58:600"
    let size = limit_header.split(',').count();
    debug_assert!(size == count_header.split(',').count());
    let mut out = Vec::with_capacity(size);

    // Pair: "100:60"
    fn scan_pair(entry: &str) -> Option<(usize, u64)> {
        let (val_str, secs_str) = entry.split_once(':')?;
        let val: usize = val_str.parse().ok()?;
        let secs: u64 = secs_str.parse().ok()?;
        Some((val, secs))
    }

    for (limit_entry, count_entry) in limit_header.split(',').zip(count_header.split(',')) {
        let (limit, limit_secs) = scan_pair(limit_entry)
            .unwrap_or_else(|| panic!("Failed to parse limit entry \"{}\".", limit_entry));
        let (count, count_secs) = scan_pair(count_entry)
            .unwrap_or_else(|| panic!("Failed to parse count entry \"{}\".", count_entry));
        debug_assert!(limit_secs == count_secs);

        let rate_usage_factor = if RateLimitType::Application == rate_limit_type {
            config.app_rate_usage_factor
        } else {
            config.method_rate_usage_factor
        };

        let limit_f32 = limit as f32;
        let scaled_burst_factor = config.burst_factor * limit_f32 / (limit_f32 + 1.0);

        let bucket = VectorTokenBucket::new(
            Duration::from_secs(limit_secs),
            limit,
            config.duration_overhead,
            scaled_burst_factor,
            rate_usage_factor,
        );
        bucket.get_tokens(count);
        out.push(bucket);
    }
    log::debug!(
        "Set buckets to {} limit, {} count.",
        limit_header,
        count_header
    );
    out
}
