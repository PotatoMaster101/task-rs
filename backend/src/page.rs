use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use crate::consts::*;

#[derive(Clone, Copy, Debug, Deserialize, Serialize, Validate)]
pub struct Page {
    pub last: Uuid,
    #[validate(range(min = MIN_PAGE_COUNT, max = MAX_PAGE_COUNT))]
    pub count: i32,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, Validate)]
pub struct TaskPage {
    pub list_id: Uuid,
    pub last: Uuid,
    #[validate(range(min = MIN_PAGE_COUNT, max = MAX_PAGE_COUNT))]
    pub count: i32,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, Validate)]
pub struct UserPage {
    pub user_id: Uuid,
    pub last: Uuid,
    #[validate(range(min = MIN_PAGE_COUNT, max = MAX_PAGE_COUNT))]
    pub count: i32,
}
