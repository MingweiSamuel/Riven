// #![deny(warnings)]

use std::convert::Infallible;

use hyper::http;
use http::{ Method, Request, Response, StatusCode };
use hyper::{ Body, Server };
use hyper::service::{ make_service_fn, service_fn };
use hyper::header::HeaderValue;
use lazy_static::lazy_static;
use tracing as log;

use riven::{ RiotApi, RiotApiConfig };
use riven::consts::Route;

lazy_static! {
    /// Create lazy static RiotApi instance.
    /// Easier than passing it around.
    pub static ref RIOT_API: RiotApi = {
        let api_key = std::env::var("RGAPI_KEY").ok()
            .or_else(|| {
                let path: std::path::PathBuf = [ env!("CARGO_MANIFEST_DIR"), "../../apikey.txt" ].iter().collect();
                std::fs::read_to_string(path).ok()
            })
            .expect("Failed to find RGAPI_KEY env var or apikey.txt.");
        RiotApi::with_config(RiotApiConfig::with_key(api_key.trim())
            .preconfig_burst())
    };
}

/// Helper to create JSON error responses.
fn create_json_response(body: &'static str, status: StatusCode) -> Response<Body> {
    let mut resp = Response::new(Body::from(body));
    *resp.status_mut() = status;
    resp.headers_mut().insert(hyper::header::CONTENT_TYPE, HeaderValue::from_static("application/json"));
    return resp;
}

/// Main request handler service.
async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {

    let (parts, body) = req.into_parts();
    let http::request::Parts { method, uri, .. } = parts;

    // Handle path.
    let path_data_opt = parse_path(&method, uri.path());
    let ( route, method_id, req_path ) = match path_data_opt {
        None => return Ok(create_json_response(
            r#"{"error":"Riot API endpoint method not found."}"#, StatusCode::NOT_FOUND)),
        Some(path_data) => path_data,
    };

    log::debug!("Request to route {:?}, method ID {:?}: {} {:?}.", route, method_id, method, req_path);

    // Convert http:request::Parts from hyper to reqwest's RequestBuilder.
    let body = match hyper::body::to_bytes(body).await {
        Err(err) => {
            log::info!("Error handling request body: {:#?}", err);
            return Ok(create_json_response(
                r#"{"error":"Failed to handle request body."}"#, StatusCode::BAD_REQUEST));
        },
        Ok(bytes) => bytes,
    };

    let req_builder = RIOT_API.request(method, route.into(), req_path)
        .body(body);

    // Send request to Riot API.
    let resp_result = RIOT_API.execute_raw(method_id, route.into(), req_builder).await;
    let resp_info = match resp_result {
        Err(err) => {
            log::info!("Riot API error: {:#?}", err.source_reqwest_error());
            return Ok(create_json_response(
                r#"{"error":"Riot API request failed."}"#, StatusCode::INTERNAL_SERVER_ERROR));
        },
        Ok(resp_info) => resp_info,
    };

    // Write output.
    let api_response = resp_info.response;
    let mut out_response = Response::default();
    *out_response.headers_mut() = api_response.headers().clone();

    // If None, write "null" to body to be extra nice.
    if resp_info.status_none {
        *out_response.body_mut() = Body::from("null");
    }
    // Otherwise copy body.
    else {
        *out_response.status_mut()  = api_response.status();

        // Using streams would be faster.
        let bytes_result = api_response.bytes().await;
        let bytes = match bytes_result {
            Err(_err) => return Ok(create_json_response(
                    r#"{"error":"Failed to get body from Riot API response."}"#, StatusCode::INTERNAL_SERVER_ERROR)),
            Ok(bytes) => bytes,
        };
        *out_response.body_mut() = Body::from((&bytes[..]).to_vec());
    }
    return Ok(out_response);
}

/// Gets the region, method_id, and Riot API path based on the given http method and path.
fn parse_path<'a>(http_method: &Method, req_path: &'a str) -> Option<( Route, &'static str, &'a str )> {

    // Split URI into region and rest of path.
    let req_path = req_path.trim_start_matches('/');
    let ( route, req_path ) = req_path.split_at(req_path.find('/')?);
    let route: Route = route.to_uppercase().parse().ok()?;

    // Find method_id for given path.
    let method_id = find_matching_method_id(http_method, req_path)?;

    return Some(( route, method_id, req_path ))
}

/// Finds the method_id given the request path.
fn find_matching_method_id(http_method: &Method, req_path: &str) -> Option<&'static str> {
    for ( endpoint_http_method, ref_path, method_id ) in &riven::meta::ALL_ENDPOINTS {
        if http_method == endpoint_http_method && paths_match(ref_path, req_path) {
            return Some(method_id);
        }
    }
    None
}

/// Checks if the request path (req_path) matches the reference path (ref_path).
fn paths_match(ref_path: &str, req_path: &str) -> bool {
    let mut ref_iter = ref_path.split('/');
    let mut req_iter = req_path.split('/');
    loop {
        let ref_seg_opt = ref_iter.next();
        let req_seg_opt = req_iter.next();
        if ref_seg_opt.is_none() != req_seg_opt.is_none() {
            return false;
        }
        if let Some(ref_seg) = ref_seg_opt {
        if let Some(req_seg) = req_seg_opt {
            if ref_seg.starts_with('{') || ref_seg == req_seg {
                continue;
            }
            return false;
        }}
        return true;
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Setup loggers.
    // env_logger::init();
    tracing_subscriber::fmt::init();

    // Trigger lazy_static.
    let _ = &*RIOT_API;

    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let make_svc = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async { Ok::<_, Infallible>(service_fn(handle_request)) }
    });

    let addr = ([ 127, 0, 0, 1 ], 3000).into();

    let server = Server::bind(&addr).serve(make_svc);

    log::info!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
