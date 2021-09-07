/// The type for a [RateLimit](super::RateLimit). Either a rate limit for the
/// entire app (`Application`) or for a specific method (`Method`).
/// Method rate limit will handle service violations.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RateLimitType {
    Application,
    Method,
}

impl RateLimitType {
    pub const fn limit_header(self) -> &'static str {
        match self {
            Self::Application => "X-App-Rate-Limit",
            Self::Method => "X-Method-Rate-Limit",
        }
    }

    pub const fn count_header(self) -> &'static str {
        match self {
            Self::Application => "X-App-Rate-Limit-Count",
            Self::Method => "X-Method-Rate-Limit-Count",
        }
    }
}
