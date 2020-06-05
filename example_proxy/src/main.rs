// #![deny(warnings)]

use std::convert::Infallible;

use hyper::service::{ make_service_fn, service_fn };
use hyper::{ Body, Method, Request, Response, Server, StatusCode };
use hyper::header::HeaderValue;
use lazy_static::lazy_static;

use riven::{ RiotApi, RiotApiConfig };
use riven::consts::Region;

lazy_static! {
    /// Create lazy static RiotApi instance.
    /// Easier than passing it around.
    pub static ref RIOT_API: RiotApi = {
        let api_key = std::env::var("RGAPI_KEY").ok()
            .or_else(|| std::fs::read_to_string("../apikey.txt").ok())
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

    if Method::GET != req.method() {
        return Ok(create_json_response(r#"{"error":"HTTP method must be GET."}"#, StatusCode::METHOD_NOT_ALLOWED));
    }

    // Handle path.
    let path_data_opt = parse_path(req.uri().path());
    let ( region, method_id, req_path ) = match path_data_opt {
        None => return Ok(create_json_response(
            r#"{"error":"Riot API endpoint method not found."}"#, StatusCode::NOT_FOUND)),
        Some(path_data) => path_data,
    };

    println!("Request to region {} path {}:\n\t{} {}", region, method_id, region, req_path);

    // Send request to Riot API.
    let query = req.uri().query().map(|s| s.to_owned());
    let resp_result = RIOT_API.get_raw_response(method_id, region.into(), req_path.to_owned(), query).await;
    let resp_info = match resp_result {
        Err(_err) => return Ok(create_json_response(
            r#"{"error":"Riot API request failed."}"#, StatusCode::INTERNAL_SERVER_ERROR)),
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

/// Gets the region, method_id, and Riot API path based on the given path.
fn parse_path<'a>(req_path: &'a str) -> Option<( Region, &'static str, &'a str )> {

    // Split URI into region and rest of path.
    let req_path = req_path.trim_start_matches('/');
    let ( region, req_path ) = req_path.split_at(req_path.find('/')?);
    let region: Region = region.to_uppercase().parse().ok()?;

    // Find method_id for given path.
    let method_id = find_matching_method_id(req_path)?;

    return Some(( region, method_id, req_path ))
}

/// Finds the method_id given the request path.
fn find_matching_method_id(req_path: &str) -> Option<&'static str> {
    for ( ref_path, method_id ) in &*riven::meta::ENDPOINT_PATH_METHODID {
        if paths_match(ref_path, req_path) {
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

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
