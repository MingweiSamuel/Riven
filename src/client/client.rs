use std::future::Future;
use std::pin::Pin;

use serde::de::DeserializeOwned;

pub type BoxFut<T> = Pin<Box<dyn Future<Output = T> + Send>>;

pub trait Client<R, E, E2> where
    R: Response<E2>,
{
    fn new() -> Self;
    fn get(&self, url_base: String, url_path: String, url_query: Option<String>,
        headers: Vec<(&'static str, String)>) -> BoxFut<Result<R, E>>;
}

// TODO: custom/generic HeaderValue trait? And for keys?
pub trait Response<E> {
    fn status_code(&self) -> u16;
    fn verison(&self) -> String;
    fn header(&self, key: String) -> Option<String>;
    fn headers_all(&self, key: String) -> Vec<String>;
    fn into_body(self) -> BoxFut<Result<String, E>>;
    fn into_json<T: DeserializeOwned + 'static>(self) -> BoxFut<Result<T, E>>;
}
