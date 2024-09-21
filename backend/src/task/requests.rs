use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use crate::consts::*;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateTaskRequest {
    pub list_id: Uuid,
    #[validate(length(min = MIN_TEXT_LENGTH, max = MAX_TEXT_LENGTH))]
    pub title: String,
    pub description: Option<String>,
    pub done: bool,
}

#[derive(Clone, Debug)]
pub struct CreateTaskWithUserRequest<'a> {
    pub user_id: Uuid,
    pub request: &'a CreateTaskRequest,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct UpdateTaskRequest {
    #[validate(length(min = MIN_TEXT_LENGTH, max = MAX_TEXT_LENGTH))]
    pub title: String,
    pub description: Option<String>,
    pub done: bool,
}

impl CreateTaskRequest {
    pub fn with_user(&self, user_id: Uuid) -> CreateTaskWithUserRequest {
        CreateTaskWithUserRequest {
            user_id,
            request: self,
        }
    }
}
