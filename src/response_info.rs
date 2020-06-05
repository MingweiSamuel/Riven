use reqwest::Response;

/// A "raw" unparsed successful response from the Riot API, for internal or advanced use cases.
pub struct ResponseInfo {
    /// The reqwest response.
    pub response: Response,
    /// The number of retries used, zero for first-try success.
    pub retries: u8,
    /// If the response has an HTTP status code indicating a `None` response (i.e. 204, 404).
    pub status_none: bool,
}
