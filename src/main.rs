use std::str::FromStr;
use tokio;
use std::net::{SocketAddr};
use axum::{
    response::IntoResponse,
    routing::get,
    Router,
    Server,
    http::StatusCode,
};

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from_str("127.0.0.1:3000").unwrap();

    let app = Router::new()
        .route("/", get(home_handler));

    println!("{}", &addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn home_handler() -> impl IntoResponse {
    (StatusCode::OK, "Index page")
}
