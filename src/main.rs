use std::str::FromStr;
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
    http::StatusCode,
    extract::{
        Query,
        Json,
    },
};
use serde::{
    Deserialize
};

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from_str("127.0.0.1:3000").unwrap();

    let app = Router::new()
        .route("/", get(home_handler))
        .route("/q", get(query_parse_handler)
            .post(body_parse_handler),
        )
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

#[derive(Debug, Deserialize)]
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