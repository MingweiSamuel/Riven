use std::iter::FromIterator;
use std::convert::TryInto;

use serde::de::DeserializeOwned;

use super::{ Client, Response, BoxFut };

impl Client for reqwest::Client {
    type Resp = reqwest::Response;
    type Err = reqwest::Error;
    fn new() -> Self {
        Self::new()
    }
    fn get(&self, url_base: String, url_path: &str, url_query: Option<&str>,
        headers: Vec<(&'static str, &str)>) -> BoxFut<Result<reqwest::Response, reqwest::Error>>
    {
        let mut url = reqwest::Url::parse(&*url_base)
            .unwrap_or_else(|_| panic!("Failed to parse url_base: \"{}\".", url_base));
        url.set_path(url_path);
        url.set_query(url_query);

        let header_iter = headers.into_iter()
            .map(|(key, value)| (
                key.try_into().unwrap_or_else(|_| panic!("Invalid header key: \"{}\".", key)),
                value.try_into().unwrap_or_else(|_| panic!("Invalid header value: \"{}\".", value)),
            ));
        let header_map = reqwest::header::HeaderMap::from_iter(header_iter);

        let fut = self.get(url)
            .headers(header_map)
            .send();
        return Box::pin(fut);
    }
}

impl Response for reqwest::Response {
    type Err = reqwest::Error;
    fn status(&self) -> u16 {
        self.status().as_u16()
    }
    fn verison(&self) -> String {
        format!("{:?}", self.version())
    }
    fn header(&self, key: &str) -> Option<String> {
        self.headers().get(key)
            .and_then(|value| value.to_str().ok())
            .map(|value| value.to_owned())
    }
    fn headers_all(&self, key: &str) -> Vec<String> {
        self.headers().get_all(key).iter()
            .filter_map(|value| value.to_str().ok())
            .map(|value| value.to_owned())
            .collect()
    }
    fn into_body(self) -> BoxFut<Result<String, reqwest::Error>> {
        //buf: Vec<u8> = Vec::with_capacity(self.content_length());
        Box::pin(self.text())
    }
    fn into_json<T: DeserializeOwned + 'static>(self) -> BoxFut<Result<T, reqwest::Error>> {
        Box::pin(self.json())
    }
}
