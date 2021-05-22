#[derive(Copy, Clone)]
pub enum RateLimitType {
    Application,
    Method,
}

impl RateLimitType {
    pub fn type_name(self) -> &'static str {
        match self {
            Self::Application => "application",
            Self::Method => "method",
        }
    }

    pub fn limit_header(self) -> &'static str {
        match self {
            Self::Application => "X-App-Rate-Limit",
            Self::Method => "X-Method-Rate-Limit",
        }
    }

    pub fn count_header(self) -> &'static str {
        match self {
            Self::Application => "X-App-Rate-Limit-Count",
            Self::Method => "X-Method-Rate-Limit-Count",
        }
    }

    /// Return if this RateLimitType should take responsibility for responses
    /// which are lacking a "X-Rate-Limit-Type" header.
    pub fn default_responsibility(self) -> bool {
        match self {
            Self::Application => true,
            Self::Method => false,
        }
    }
}
