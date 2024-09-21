use actix_web::{HttpResponse, Responder};
use crate::user::*;

pub async fn get_user(user: User) -> impl Responder {
    HttpResponse::Ok().json(user)
}
