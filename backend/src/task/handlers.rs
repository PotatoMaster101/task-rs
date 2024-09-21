use actix_web::{HttpResponse, Responder, web, ResponseError};
use uuid::Uuid;
use validator::Validate;
use crate::error::*;
use crate::page::*;
use crate::repo::*;
use crate::task::{repo::*, requests::*};
use crate::user::*;

pub async fn create_task(task: web::Json<CreateTaskRequest>, repo: web::Data<TaskRepository>, user: AuthedUser) -> impl Responder {
    if let Err(err) = task.validate() {
        return ApiError::BadParam(&err.to_string()).error_response();
    }

    let result = repo.create(&task.with_user(user.user.id)).await;
    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(err) => HttpResponse::BadRequest().body(format!("{err:?}"))
    }
}

pub async fn delete_task(id: web::Path<Uuid>, repo: web::Data<TaskRepository>) -> impl Responder {
    let result = repo.delete(&id).await;
    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(err) => HttpResponse::BadRequest().body(format!("{err:?}"))
    }
}

pub async fn get_task(id: web::Path<Uuid>, repo: web::Data<TaskRepository>) -> impl Responder {
    let result = repo.get(&id).await;
    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(err) => HttpResponse::BadRequest().body(format!("{err:?}"))
    }
}

pub async fn get_tasks(page: web::Query<TaskPage>, repo: web::Data<TaskRepository>) -> impl Responder {
    if page.validate().is_err() {
        return ApiError::BadPaginate(page.count).error_response();
    }

    let result = repo.paginate(&page).await;
    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(err) => HttpResponse::BadRequest().body(format!("{err:?}"))
    }
}

pub async fn update_task(id: web::Path<Uuid>, task: web::Json<UpdateTaskRequest>, repo: web::Data<TaskRepository>) -> impl Responder {
    if let Err(err) = task.validate() {
        return ApiError::BadParam(&err.to_string()).error_response();
    }

    let result = repo.update(&id, &task).await;
    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(err) => HttpResponse::BadRequest().body(format!("{err:?}"))
    }
}
