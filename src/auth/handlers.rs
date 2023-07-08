use axum::{
    Extension,
    response::{IntoResponse},
    http::StatusCode,
    Router,
    extract::{Path},
};
use std::sync::Arc;
use super::{
    User,
    UserManager,
};
use crate::shared::ErrorMessage;
use serde::Serialize;
use std::convert::Infallible;
use axum::routing::get;

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

async fn user_list_handler(e: Extension<Arc<UserManager>>) -> Result<impl IntoResponse, Infallible> {
    let user_manager: Arc<UserManager> = e.0.clone();
    match user_manager.all().await {
        Ok(users) => Ok((StatusCode::OK, axum::response::Json(UserOrError::Users(UserList { users })))),
        Err(_) => Ok((StatusCode::INTERNAL_SERVER_ERROR, axum::response::Json(UserOrError::Error(ErrorMessage { message: "Internal server error".to_string() })))),
    }
}

async fn get_user(e: Extension<Arc<UserManager>>, Path(user_id): Path<i32>) -> Result<impl IntoResponse, Infallible> {
    let user_manager: Arc<UserManager> = e.0.clone();
    match user_manager.get(user_id).await {
        Ok(user) => Ok((StatusCode::OK, axum::response::Json(UserSingleOrError::User(user)))),
        Err(_) => Ok((StatusCode::NO_CONTENT, axum::response::Json(UserSingleOrError::Error(ErrorMessage { message: "No user found".into() }))))
    }
}

pub fn create_router(um: Arc<UserManager>) -> Router {
    Router::new()
        .route("/", get(user_list_handler))
        .route("/:id", get(get_user))
        .layer(Extension(um.clone()))
}