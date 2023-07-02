use std::str::FromStr;
use rand::Rng;
use tokio;
use std::net::{SocketAddr};
use axum::{
    response::IntoResponse,
    routing::{
        get,
        post,
    },
    Router,
    Server,
    http::{
        header,
        StatusCode,
    },
    extract::{
        Query,
        Json,
    },
};
use axum::http::Response;
use serde::{
    Deserialize
};
use axum_extra::extract::cookie::CookieJar;
use rand::rngs::ThreadRng;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from_str("127.0.0.1:3000").unwrap();

    let app = Router::new()
        .route("/", get(home_handler))
        .route("/q", get(query_parse_handler).post(body_parse_handler))
        .route("/c", get(cookie_parse_handler))
        .route("/cq", get(cookie_and_query))
        .route("/both", post(q_and_body))
        .route("/set_cookie", get(set_cookie_handler))
        ;

    println!("{}", &addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn home_handler() -> impl IntoResponse {
    (StatusCode::OK, "Index page")
}

#[derive(Debug, Deserialize, Clone, Copy, Default)]
struct QuerySample1 {
    id: i32,
}

async fn query_parse_handler(query: Query<QuerySample1>) -> impl IntoResponse {
    println!("{:?}", query);
    (StatusCode::OK, format!("id is {}", query.id))
}

async fn body_parse_handler(body: Json<QuerySample1>) -> impl IntoResponse {
    println!("{:?}", body);
    (StatusCode::OK, format!("id is {}", body.id))
}

// async fn q_and_body(body: Json<QuerySample1>, query: Query<QuerySample1>) -> impl IntoResponse { // compile error
async fn q_and_body(query: Query<QuerySample1>, body: Json<QuerySample1>) -> impl IntoResponse {
    println!("{:?}", body);
    println!("{:?}", query);
    (StatusCode::OK, format!("id is {}, {}", body.id, query.id))
}

async fn cookie_parse_handler(cookie: CookieJar) -> impl IntoResponse {
    println!("{:?}", cookie);
    if let Some(sid) = cookie.get("sid") {
        (StatusCode::OK, format!("id is {}", sid))
    } else {
        (StatusCode::UNAUTHORIZED, "not authorized".into())
    }
}

async fn cookie_and_query(cookie: CookieJar, query: Query<QuerySample1>) -> impl IntoResponse {
    println!("{:?}", cookie);
    println!("{:?}", query);
    let qid: i32 = query.id;
    if let Some(sid) = cookie.get("sid") {
        (StatusCode::OK, format!("sid(cookie) is {}, id(query) is {}", sid, qid))
    } else {
        (StatusCode::UNAUTHORIZED, "not authorized".into())
    }
}

async fn set_cookie_handler() -> impl IntoResponse {
    let mut rng: ThreadRng = rand::thread_rng();
    let sid: i32 = rng.gen_range(0..100000);

    let response: Response<String> = Response::builder()
        // .header("X-Custom-Foo", "Bar")
        // .header(header::SET_COOKIE, format!("sid={}; SameSite=Strict", sid))
        .header(header::SET_COOKIE, format!("sid={};", sid))
        .body(format!("your sid is {}", sid))
        .unwrap();

    (StatusCode::OK, response.into_parts())
}