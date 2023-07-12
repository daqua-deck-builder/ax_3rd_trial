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
use axum::response::Response;
use crate::auth::{CreateUser, UserForAuthenticate};
use bcrypt::{hash, verify, DEFAULT_COST};
use futures::future::err;
use crate::auth;

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
        Ok(users) => Ok((StatusCode::OK, Json(UserOrError::Users(UserList { users })))),
        Err(_) => Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(UserOrError::Error(ErrorMessage { message: "Internal server error".to_string() })))),
    }
}

async fn get_user_handler(e: Extension<Arc<UserManager>>, Path(user_id): Path<i32>) -> Result<impl IntoResponse, Infallible> {
    let user_manager: Arc<UserManager> = e.0.clone();
    match user_manager.get(user_id).await {
        Ok(user) => Ok((StatusCode::OK, Json(UserSingleOrError::User(user)))),
        Err(_) => Ok((StatusCode::NO_CONTENT, Json(UserSingleOrError::Error(ErrorMessage { message: "No user found".into() }))))
    }
}

async fn create_user_handler(e: Extension<Arc<UserManager>>, create_user: Json<CreateUser>) -> impl IntoResponse {
    if let Err(error) = create_user.valid_username() {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(SimpleResult::ResultReason(ResultWithReason { success: false, reason: Some(error) })));
    }

    let user_manager: Arc<UserManager> = e.0.clone();
    let hashed_password = hash(&create_user.password, DEFAULT_COST).unwrap();

    println!("{} {}", create_user.username, hashed_password);
    match user_manager.create(&CreateUser { username: create_user.username.clone(), password: hashed_password.clone() }).await {
        Ok(user) => {
            (StatusCode::CREATED, Json(SimpleResult::Simple(Simple { success: true })))
        }
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(SimpleResult::Simple(Simple { success: false })))
        }
    }
}

#[derive(Serialize)]
enum ResponseBody {
    User(UserList),
    Error(ErrorMessage),
    NoContent,
}

pub struct MyResponse(StatusCode, Option<Json<ResponseBody>>);

impl IntoResponse for MyResponse {
    fn into_response(self) -> Response {
        match self.1 {
            Some(body) => (self.0, body).into_response(),
            None => (self.0, String::from("")).into_response(),
        }
    }
}

impl MyResponse {
    fn users(users: Vec<User>) -> MyResponse {
        MyResponse(StatusCode::OK, Some(Json(ResponseBody::User(UserList { users }))))
    }
    fn error_from_str(error_message: String) -> MyResponse {
        MyResponse(StatusCode::OK, Some(Json(ResponseBody::Error(ErrorMessage { message: error_message }))))
    }
    fn no_content() -> MyResponse {
        MyResponse(StatusCode::NO_CONTENT, None)
    }
    fn server_error() -> MyResponse {
        MyResponse(StatusCode::INTERNAL_SERVER_ERROR, None)
    }
}

async fn delete_user_handler(e: Extension<Arc<UserManager>>, Path(user_id): Path<i32>) -> impl IntoResponse {
    let user_manager = e.0.clone();
    match user_manager.delete(user_id).await {
        Ok(true) => {
            match user_manager.all().await {
                Ok(users) => MyResponse::users(users),
                Err(_) => MyResponse::error_from_str("unknown error".into()),
            }
        }
        Ok(false) => MyResponse::no_content(),
        _ => MyResponse::server_error(),
    }
}

async fn try_login(e: Extension<Arc<UserManager>>, login_info: Json<CreateUser>) -> impl IntoResponse {
    let user_manager = e.0.clone();
    match user_manager.authenticate(&login_info.username, &login_info.password).await {
        Ok(true) => {
            (StatusCode::OK, Json(SimpleResult::Simple(Simple { success: true })))
        }
        Ok(false) => {
            (StatusCode::OK, Json(SimpleResult::Simple(Simple { success: false })))
        }
        Err(_) => {
            (StatusCode::OK, Json(SimpleResult::Simple(Simple { success: false })))
        }
    }
}

pub fn create_router(um: Arc<UserManager>) -> Router {
    Router::new()
        .route("/", get(user_list_handler))
        .route("/:id",
               get(get_user_handler)
                   .delete(delete_user_handler),
        )
        .route("/create", post(create_user_handler))
        .route("/login", post(try_login))
        .layer(Extension(um.clone()))
}