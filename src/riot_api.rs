use crate::*;
use crate::consts::Region;
use crate::req::RequesterManager;

pub struct RiotApi<'a> {
    pub requester_manager: RequesterManager<'a>,
    _private: (),
}

impl<'a> RiotApi<'a> {
    pub fn with_config(config: RiotApiConfig<'a>) -> Self {
        let req_man = RequesterManager::new(config);
        Self {
            requester_manager: req_man,
            _private: (),
        }
    }

    pub fn with_key(api_key: &'a str) -> Self {
        Self::with_config(RiotApiConfig::with_key(api_key))
    }

    pub async fn get<T: serde::de::DeserializeOwned>(
        &'a self, method_id: &'a str, region: Region, path: &str,
        query: Option<&str>) -> Result<Option<T>, reqwest::Error>
    {
        self.requester_manager.get(method_id, region, path, query).await
    }
}
