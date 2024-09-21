pub mod handlers;
pub mod repo;
pub mod requests;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::{ContextV7, Timestamp, Uuid};

#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
pub struct Task {
    pub id: Uuid,
    pub list_id: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub title: String,
    pub description: Option<String>,
    pub done: bool,
}
