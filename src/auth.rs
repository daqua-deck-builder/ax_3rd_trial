use std::sync::Arc;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool};
use sqlx::pool::PoolConnection;
use sqlx::postgres::Postgres;
use sqlx::postgres::PgPoolOptions;

#[derive(Serialize, FromRow, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

impl User {
    fn new(username: String, password: String) -> User {
        User {
            id: 1,
            username,
            password,
        }
    }
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
}


pub struct UserManager {
    pool: Pool<Postgres>,
}

impl UserManager {
    pub fn new(pool: Pool<Postgres>) -> UserManager {
        UserManager { pool }
    }

    pub async fn all(&self) -> Result<Vec<User>, sqlx::Error> {
        println!("in create method");

        let value: Vec<User> = sqlx::query_as("select * from users;")
            .fetch_all(&self.pool)
            .await?;

        Ok(value)
    }
    pub async fn create(&self, createUser: &CreateUser) {
        todo!()
    }
    pub fn find_by_id(id: i32) -> Result<User, String> {
        todo!()
    }
    pub fn delete(id: i32) -> bool {
        todo!()
    }
}