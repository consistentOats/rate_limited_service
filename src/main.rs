mod rate_limiter;

use uuid::{uuid, Uuid};
use warp::{Filter, hyper::{HeaderMap, StatusCode}};

pub const POST_VAULT_FUNCTION_ID: Uuid = uuid!("4669c1c3-f18c-4fba-9092-ed48b3053c82");
pub const GET_VAULT_ITEMS_FUNCTION_ID: Uuid  = uuid!("3b04da2a-bf73-4cf2-a2f1-efa37cc707d6");
pub const PUT_VAULT_ITEM_FUNCTION_ID: Uuid  = uuid!("b5d2da56-af7b-41df-bd2f-f6c3dd2cc482");

#[tokio::main]
async fn main() {
    let post_vault_route = warp::path("vault")
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::header::headers_cloned())
        .map(|headers| post_vault(headers));
    
    let get_vault_items_route = warp::path!("vault" / "items")
        .and(warp::path::end())
        .and(warp::get())
        .and(warp::header::headers_cloned())
        .map(|headers| get_vault_items(headers));

    let put_vault_item_route = warp::path!("vault" / "items" / String)
        .and(warp::path::end())
        .and(warp::put())
        .and(warp::header::headers_cloned())
        .map(|id, headers| put_vault_item(headers, id));

    let routes = post_vault_route
        .or(get_vault_items_route)
        .or(put_vault_item_route);

    warp::serve(routes)
        .run(([127,0,0,1], 8080))
        .await;
}

// POST "/vault"
pub fn post_vault(headers: HeaderMap) -> Result<impl warp::Reply, warp::http::Error> {
    Ok(warp::reply::with_status(warp::reply(), StatusCode::OK))
}

// GET "/vault/items"
pub fn get_vault_items(headers: HeaderMap) -> Result<impl warp::Reply, warp::http::Error> {
    Ok(warp::reply::with_status(warp::reply(), StatusCode::OK))
}

// PUT "/vault/items/<:id>
pub fn put_vault_item(headers: HeaderMap, id: String) -> Result<impl warp::Reply, warp::http::Error> {
    Ok(warp::reply::with_status(warp::reply(), StatusCode::OK))
}