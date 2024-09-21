use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use crate::consts::*;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateTaskListRequest {
    pub user_id: Uuid,
    #[validate(length(min = MIN_TEXT_LENGTH, max = MAX_TEXT_LENGTH))]
    pub title: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct UpdateTaskListRequest {
    #[validate(length(min = MIN_TEXT_LENGTH, max = MAX_TEXT_LENGTH))]
    pub title: String,
}
