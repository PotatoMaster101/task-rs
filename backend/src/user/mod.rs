pub mod handlers;
pub mod repo;
pub mod requests;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub email: String,
    pub auth0_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthedUser {
    pub user: User,
    pub roles: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthedAdmin(pub AuthedUser);
