pub mod handlers;
pub mod repo;
pub mod requests;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::{uuid, Uuid};

const ADMIN_ROLE_ID: &str = "79c63692-a983-45bf-8e28-21ed0dc84b19";
const USER_ROLE_ID: &str = "853e018a-07fe-4b1c-863f-5203d5589cab";

#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub role_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub email: String,
    pub auth0_id: String,
}

#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
pub struct Role {
    pub id: Uuid,
    pub name: String,
}

impl User {
    pub fn is_admin(&self) -> bool {
        self.role_id == uuid!(ADMIN_ROLE_ID)
    }
}
