mod auth;

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
        ws::{
            Message,
            WebSocket,
            WebSocketUpgrade,
        },
    },
};
use axum::http::Response;
use serde::{
    Deserialize,
    Serialize,
};
use axum_extra::extract::cookie::CookieJar;
use futures::{SinkExt, StreamExt};
use rand::rngs::ThreadRng;
use sqlx::Postgres;
use uuid::Uuid;
use auth::UserManager;
use crate::auth::CreateUser;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@192.168.33.10/testdb1").await;
    let pool: sqlx::Pool<Postgres> = pool.unwrap();
    let mut user_manager: UserManager = UserManager::new(pool);
    let users: Result<Vec<auth::User>, sqlx::Error> = user_manager.all().await;
    users.unwrap().iter().for_each(|u: &auth::User| { println!("{} {} {}", u.id, u.username, u.password) });

    let addr = SocketAddr::from_str("127.0.0.1:3000").unwrap();

    let app = Router::new()
        .route("/", get(home_handler))
        .route("/q", get(query_parse_handler).post(body_parse_handler))
        .route("/c", get(cookie_parse_handler))
        .route("/cq", get(cookie_and_query))
        .route("/both", post(q_and_body))
        .route("/set_cookie", get(set_cookie_handler))
        .route("/api/item1.json", get(json_handler1))
        .route("/websocket", get(websocket_handler))
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

#[derive(Serialize)]
struct WSConnectionResult {
    id: String,
    message_type: String,
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    println!("ws access");
    ws.on_upgrade(|socket| websocket(socket))
}

async fn websocket(stream: WebSocket) {
    let (mut sender, mut receiver) = stream.split();

    println!("Client connected");

    // 接続開始時にUUIDを生成しクライアントに投げる
    let connection_result = WSConnectionResult {
        id: Uuid::new_v4().to_string(),
        message_type: "connected".into(),
    };
    sender.send(Message::Text(serde_json::to_string(&connection_result).unwrap())).await.unwrap();  // ブロッキング注意

    while let Some(result) = receiver.next().await {
        match result {
            Ok(message) => {
                if let Message::Text(message_text) = message {
                    println!("{}", message_text);
                }
            }
            Err(e) => {
                println!("Error in websocket: {:?}", e);
                break;
            }
        }
    }

    println!("Client disconnected")
}


#[derive(Debug, Deserialize, Serialize, Clone, Copy, Default)]
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

async fn json_handler1() -> impl IntoResponse {
    let item = QuerySample1 {
        id: 500
    };
    (StatusCode::OK, Json(item))
}