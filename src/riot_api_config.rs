#[derive(Debug)]
pub struct RiotApiConfig {
    pub api_key: String,
    pub retries: u8,
}

impl RiotApiConfig {
    pub fn with_key<T: Into<String>>(api_key: T) -> Self {
        Self {
            api_key: api_key.into(),
            retries: 3 // TODO defaults.
        }
    }
}
