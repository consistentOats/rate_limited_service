use std::{sync::{Mutex, Arc}, collections::HashMap};

use chrono::{DateTime, Duration, Utc};
use warp::{Filter, hyper::{Response, HeaderMap, StatusCode}};
use sha256;

const POST_VAULT_ROUTE: &str = "POST /vault";
const GET_VAULT_ITEMS_ROUTE: &str = "GET /vault/items";
const PUT_VAULT_ITEM_ROUTE: &str = "PUT /vault/items/<:id>";

const POST_VAULT_RATE_LIMIT: i32 = 3;
const GET_VAULT_ITEMS_RATE_LIMIT: i32 = 1200;
const PUT_VAULT_ITEM_RATE_LIMIT: i32 = 60;

#[tokio::main]
async fn main() {
    let rate_limiter = RateLimiter::new();
    let rate_limiter_filter = warp::any().map(move || rate_limiter.clone());

    let post_vault_route = warp::path("vault")
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::header::headers_cloned())
        .and(rate_limiter_filter.clone())
        .map(|headers, rate_limiter| post_vault(rate_limiter, headers));
    
    let get_vault_items_route = warp::path!("vault" / "items")
        .and(warp::path::end())
        .and(warp::get())
        .and(warp::header::headers_cloned())
        .and(rate_limiter_filter.clone())
        .map(|headers, rate_limiter| get_vault_items(rate_limiter, headers));

    let put_vault_item_route = warp::path!("vault" / "items" / String)
        .and(warp::path::end())
        .and(warp::put())
        .and(warp::header::headers_cloned())
        .and(rate_limiter_filter.clone())
        .map(|id, headers, rate_limiter| put_vault_item(rate_limiter, headers, id));

    let routes = post_vault_route
        .or(get_vault_items_route)
        .or(put_vault_item_route);

    warp::serve(routes)
        .run(([127,0,0,1], 8080))
        .await;
}

// POST "/vault"
pub fn post_vault(rate_limiter: RateLimiter, headers: HeaderMap) -> Result<warp::reply::Response, warp::http::Error> {
    let bearer_token = match headers.get("Authorization").map(|token| token.to_str()) {
        Some(Ok(token)) => token.to_string(),
        _ => return unauthorized_reply(),
    };

    match rate_limiter.log_usage(POST_VAULT_ROUTE, bearer_token, RateLimit::new(POST_VAULT_RATE_LIMIT)) {
        Ok(_) => ok_reply(),
        Err(err) => rate_limited_reply(err),
    }
}

// GET "/vault/items"
pub fn get_vault_items(rate_limiter: RateLimiter, headers: HeaderMap) -> Result<warp::reply::Response, warp::http::Error> {
    let bearer_token = match headers.get("Authorization").map(|token| token.to_str()) {
        Some(Ok(token)) => token.to_string(),
        _ => return unauthorized_reply(),
    };

    match rate_limiter.log_usage(GET_VAULT_ITEMS_ROUTE, bearer_token, RateLimit::new(GET_VAULT_ITEMS_RATE_LIMIT)) {
        Ok(_) => ok_reply(),
        Err(err) => rate_limited_reply(err),
    }
}

// PUT "/vault/items/<:id>
pub fn put_vault_item(rate_limiter: RateLimiter, headers: HeaderMap, id: String) -> Result<warp::reply::Response, warp::http::Error> {
    let bearer_token = match headers.get("Authorization").map(|token| token.to_str()) {
        Some(Ok(token)) => token.to_string(),
        _ => return unauthorized_reply(),
    };

    match rate_limiter.log_usage(PUT_VAULT_ITEM_ROUTE, bearer_token, RateLimit::new(PUT_VAULT_ITEM_RATE_LIMIT)) {
        Ok(_) => ok_reply(),
        Err(err) => rate_limited_reply(err),
    }
}

fn unauthorized_reply() -> Result<warp::reply::Response, http::Error> {
    Response::builder().status(StatusCode::UNAUTHORIZED).body("".into())
}

fn ok_reply() -> Result<warp::reply::Response, http::Error> {
    Response::builder().status(StatusCode::OK).body("".into())
}

fn rate_limited_reply(err: RateLimitedError) -> Result<warp::reply::Response, http::Error> {
    Response::builder().status(StatusCode::TOO_MANY_REQUESTS).body("".into())
}

#[derive(Debug, Clone)]
pub struct RateLimiter {
    usage_counter: Arc<HashMap<String, (Mutex<i32>, DateTime<Utc>)>>
}

impl RateLimiter {
    pub fn new() -> Self {
        RateLimiter { usage_counter: Arc::new(HashMap::new()) }
    }

    pub fn log_usage(self, route: &str, bearer_token: String, rate_limit: RateLimit) -> Result<(), RateLimitedError> {
        // bearer token cannot be stored on it's own as it is a security issue
        let hashed_key = sha256::digest(route.to_string() + &bearer_token);
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub struct RateLimit {
    pub limit: i32, 
    pub duration: Duration,
}

impl RateLimit {
    pub fn new(limit: i32) -> Self {
        // duration defaults to 1 minute
        RateLimit { 
            limit, 
            duration: Duration::minutes(1)
        }
    }
}

#[derive(Debug, Clone)]
pub struct RateLimitedError {

}

impl RateLimitedError {
    pub fn new() -> Self {
        RateLimitedError {  }
    }
}