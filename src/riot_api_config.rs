#[derive(Debug)]
pub struct RiotApiConfig<'a> {
    pub api_key: &'a str,
    pub retries: u8,
}

impl<'a> RiotApiConfig<'a> {
    pub fn with_key(api_key: &'a str) -> Self {
        Self {
            api_key: api_key,
            retries: 3 // TODO defaults.
        }
    }
}
