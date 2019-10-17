use crate::RiotApiConfig;
use crate::consts::Region;
use crate::req::RequesterManager;

pub struct RiotApi<'a> {
    requester_manager: RequesterManager<'a>,
}

impl<'a> RiotApi<'a> {
    pub fn with_config(config: RiotApiConfig<'a>) -> Self {
        Self {
            requester_manager: RequesterManager::new(config),
        }
    }

    pub fn with_key(api_key: &'a str) -> Self {
        Self::with_config(RiotApiConfig::with_key(api_key))
    }

    pub async fn get<T: serde::de::DeserializeOwned>(
        &'a self, method_id: &'a str, region: &'a Region<'a>, relative_url: &'_ str,
        query: &[(&'_ str, &'_ str)]) -> Result<Option<T>, reqwest::Error>
    {
        self.requester_manager.get(method_id, region, relative_url, query).await
    }
}
