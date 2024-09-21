use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use crate::consts::*;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateTaskListRequest {
    #[validate(length(min = MIN_TEXT_LENGTH, max = MAX_TEXT_LENGTH))]
    pub title: String,
}

#[derive(Clone, Debug)]
pub struct CreateTaskListWithUserRequest<'a> {
    pub user_id: Uuid,
    pub request: &'a CreateTaskListRequest,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct UpdateTaskListRequest {
    #[validate(length(min = MIN_TEXT_LENGTH, max = MAX_TEXT_LENGTH))]
    pub title: String,
}

impl CreateTaskListRequest {
    pub fn with_user(&self, user_id: Uuid) -> CreateTaskListWithUserRequest {
        CreateTaskListWithUserRequest {
            user_id,
            request: self,
        }
    }
}
