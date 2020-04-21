use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;

use serde::de::DeserializeOwned;

pub type BoxFut<T> = Pin<Box<dyn Future<Output = T> + Send>>;

pub trait Client {
    type Resp: Response;
    type Err: Debug;
    fn new() -> Self;
    fn get(&self, url_base: String, url_path: &str, url_query: Option<&str>,
        headers: Vec<(&'static str, &str)>) -> BoxFut<Result<Self::Resp, <Self::Resp as Response>::Err>>;
}

// TODO: custom/generic HeaderValue trait? And for keys?
pub trait Response {
    type Err: Debug;
    fn status(&self) -> u16;
    fn verison(&self) -> String;
    fn header(&self, key: &str) -> Option<String>;
    fn headers_all(&self, key: &str) -> Vec<String>;
    fn into_body(self) -> BoxFut<Result<String, Self::Err>>;
    fn into_json<T: DeserializeOwned + 'static>(self) -> BoxFut<Result<T, Self::Err>>;
}
