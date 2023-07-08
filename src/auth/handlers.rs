use axum::{
    Extension,
    response::{IntoResponse},
    http::StatusCode,
};
use std::sync::Arc;
use super::{
    User,
    UserManager,
};
use crate::shared::ErrorMessage;
use serde::Serialize;
use std::convert::Infallible;

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

pub async fn user_list_handler(e: Extension<Arc<UserManager>>) -> Result<impl IntoResponse, Infallible> {
    let user_manager: Arc<UserManager> = e.0.clone();
    match user_manager.all().await {
        Ok(users) => Ok((StatusCode::OK, axum::response::Json(UserOrError::Users(UserList { users })))),
        Err(_) => Ok((StatusCode::INTERNAL_SERVER_ERROR, axum::response::Json(UserOrError::Error(ErrorMessage { message: "Internal server error".to_string() })))),
    }
}