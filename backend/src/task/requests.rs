use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use crate::consts::*;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateTaskRequest {
    pub list_id: Uuid,
    pub user_id: Uuid,
    #[validate(length(min = MIN_TEXT_LENGTH, max = MAX_TEXT_LENGTH))]
    pub title: String,
    pub description: Option<String>,
    pub done: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct UpdateTaskRequest {
    #[validate(length(min = MIN_TEXT_LENGTH, max = MAX_TEXT_LENGTH))]
    pub title: String,
    pub description: Option<String>,
    pub done: bool,
}
