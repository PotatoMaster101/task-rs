use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::consts::*;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email, length(min = MIN_TEXT_LENGTH, max = MAX_TEXT_LENGTH))]
    pub email: String,
    #[validate(length(min = MIN_TEXT_LENGTH, max = MAX_TEXT_LENGTH))]
    pub auth0_id: String,
}
