use axum::{
    Extension,
    response::{IntoResponse},
    http::StatusCode,
    Router,
    extract::{Path, Json},
    routing::{get, post},
};
use std::sync::Arc;
use super::{
    User,
    UserManager,
};
use crate::shared::ErrorMessage;
use serde::Serialize;
use std::convert::Infallible;
use crate::auth::CreateUser;
use bcrypt::{hash, verify, DEFAULT_COST};

#[derive(Serialize)]
pub struct UserList {
    users: Vec<User>,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum UserOrError {
    Users(UserList),
    Error(ErrorMessage),
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum UserSingleOrError {
    User(User),
    Error(ErrorMessage),
}

#[derive(Serialize)]
struct Simple {
    success: bool,
}

#[derive(Serialize)]
struct ResultWithReason {
    success: bool,
    reason: Option<Vec<String>>,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum SimpleResult {
    Simple(Simple),
    ResultReason(ResultWithReason),
}

async fn user_list_handler(e: Extension<Arc<UserManager>>) -> Result<impl IntoResponse, Infallible> {
    let user_manager: Arc<UserManager> = e.0.clone();
    match user_manager.all().await {
        Ok(users) => Ok((StatusCode::OK, axum::response::Json(UserOrError::Users(UserList { users })))),
        Err(_) => Ok((StatusCode::INTERNAL_SERVER_ERROR, axum::response::Json(UserOrError::Error(ErrorMessage { message: "Internal server error".to_string() })))),
    }
}

async fn get_user_handler(e: Extension<Arc<UserManager>>, Path(user_id): Path<i32>) -> Result<impl IntoResponse, Infallible> {
    let user_manager: Arc<UserManager> = e.0.clone();
    match user_manager.get(user_id).await {
        Ok(user) => Ok((StatusCode::OK, axum::response::Json(UserSingleOrError::User(user)))),
        Err(_) => Ok((StatusCode::NO_CONTENT, axum::response::Json(UserSingleOrError::Error(ErrorMessage { message: "No user found".into() }))))
    }
}

async fn create_user_handler(e: Extension<Arc<UserManager>>, create_user: Json<CreateUser>) -> impl IntoResponse {
    if let Err(error) = create_user.valid_username() {
        return (StatusCode::INTERNAL_SERVER_ERROR, axum::response::Json(SimpleResult::ResultReason(ResultWithReason { success: false, reason: Some(error) })));
    }

    let user_manager: Arc<UserManager> = e.0.clone();
    let hashed_password = hash(&create_user.password, DEFAULT_COST).unwrap();

    println!("{} {}", create_user.username, hashed_password);
    match user_manager.create(&CreateUser { username: create_user.username.clone(), password: hashed_password.clone() }).await {
        Ok(user) => {
            (StatusCode::CREATED, axum::response::Json(SimpleResult::Simple(Simple { success: true })))
        }
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, axum::response::Json(SimpleResult::Simple(Simple { success: false })))
        }
    }
}

pub fn create_router(um: Arc<UserManager>) -> Router {
    Router::new()
        .route("/", get(user_list_handler))
        .route("/:id", get(get_user_handler))
        .route("/create", post(create_user_handler))
        .layer(Extension(um.clone()))
}