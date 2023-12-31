pub mod handlers;

use std::sync::Arc;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool};
use sqlx::pool::PoolConnection;
use sqlx::postgres::Postgres;
use sqlx::postgres::PgPoolOptions;
use bcrypt::{hash, verify, DEFAULT_COST};

#[derive(Serialize, FromRow, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    // pub password: String,
}

#[derive(Serialize, FromRow, Debug)]
pub struct UserForAuthenticate {
    // pub id: i32,
    pub username: String,
    pub password: String,
}

impl User {
    fn new(username: String, password: String) -> User {
        User {
            id: 1,
            username,
            // password,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
}

impl CreateUser {
    pub fn valid_username(&self) -> Result<(), Vec<String>> {
        let mut errors: Vec<String> = Vec::new();
        if !self.username.chars().all(|c| c.is_ascii_alphanumeric()) {
            errors.push("invalid letter".into())
        }

        match errors.len() {
            0 => Ok(()),
            _ => Err(errors)
        }
    }
}

pub struct UserManager {
    pool: Pool<Postgres>,
}

impl UserManager {
    pub fn new(pool: Pool<Postgres>) -> UserManager {
        UserManager { pool }
    }

    pub async fn all(&self) -> Result<Vec<User>, sqlx::Error> {
        println!("[all]");

        let value: Vec<User> = sqlx::query_as("select * from users;")
            .fetch_all(&self.pool)
            .await?;

        Ok(value)
    }
    pub async fn get(&self, id: i32) -> Result<User, sqlx::Error> {
        println!("[get]");

        let user: User = sqlx::query_as("select * from users where id = $1;")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }

    pub async fn create(&self, create_user: &CreateUser) -> Result<User, sqlx::Error> {
        println!("[create]");

        let user: User = sqlx::query_as("insert into users (username, password) values ($1, $2) returning id, username;")
            .bind(&create_user.username)
            .bind(&create_user.password)
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }

    pub async fn delete(&self, id: i32) -> Result<bool, sqlx::Error> {
        println!("[delete]");

        let result: u64 = sqlx::query("delete from users where id = $1;")
            .bind(id)
            .execute(&self.pool)
            .await?
            .rows_affected();

        Ok(result > 0)
    }

    pub async fn authenticate(&self, username: &String, password: &String) -> Result<bool, sqlx::Error> {
        println!("[authenticate]");

        match sqlx::query_as::<_, UserForAuthenticate>("select username, password from users where username = $1;")
            .bind(&username)
            .fetch_optional(&self.pool)
            .await {
            Ok(Some(user)) => {
                match verify(password, &user.password) {
                    Ok(password_matches) => Ok(password_matches),
                    Err(_) => Ok(false),
                }
            }
            Ok(None) => Ok(false), // if there's no such user, return false
            Err(e) => Err(e), // if there's a database error, propagate it
        }
    }
}