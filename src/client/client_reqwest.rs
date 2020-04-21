use std::iter::FromIterator;
use std::convert::TryInto;

use serde::de::DeserializeOwned;

use super::{ Client, Response, BoxFut };

impl Client<reqwest::Response, reqwest::Error, reqwest::Error> for reqwest::Client {
    fn new() -> Self {
        Self::new()
    }
    fn get(&self, url_base: String, url_path: String, url_query: Option<String>,
        headers: Vec<(&'static str, String)>) -> BoxFut<Result<reqwest::Response, reqwest::Error>>
    {
        #[cfg(feature = "nightly")] let url_query = url_query.as_deref();
        #[cfg(not(feature = "nightly"))] let url_query = url_query.as_ref().map(|s| s.as_ref());

        let mut url = reqwest::Url::parse(&*url_base)
            .unwrap_or_else(|_| panic!("Failed to parse url_base: \"{}\".", url_base));
        url.set_path(&*url_path);
        url.set_query(url_query);

        let header_iter = headers.into_iter()
            .map(|(key, value)| (
                key.try_into().unwrap_or_else(|_| panic!("Invalid header key: \"{}\".", &key)),
                // Makes a copy.
                (&value).try_into().unwrap_or_else(|_| panic!("Invalid header value: \"{}\".", &value)),
            ));
        let header_map = reqwest::header::HeaderMap::from_iter(header_iter);

        let fut = self.get(url)
            .headers(header_map)
            .send();
        return Box::pin(fut);
    }
}

impl Response<reqwest::Error> for reqwest::Response {
    fn status_code(&self) -> u16 {
        self.status().as_u16()
    }
    fn verison(&self) -> String {
        format!("{:?}", self.version())
    }
    fn header(&self, key: String) -> Option<String> {
        self.headers().get(key)
            .and_then(|value| value.to_str().ok())
            .map(|value| value.to_owned())
    }
    fn headers_all(&self, key: String) -> Vec<String> {
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
