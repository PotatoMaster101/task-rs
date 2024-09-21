mod auth;
mod consts;
mod error;
mod page;
mod repo;
mod task_list;
mod task;
mod user;

use std::env;
use actix_web::{App, HttpServer, middleware::Logger, web};
use actix_web_httpauth::extractors::bearer::Config;
use crate::task::handlers::*;
use crate::task_list::handlers::*;
use crate::user::handlers::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let db = env::var("DATABASE_URL").expect("missing DATABASE_URL");
    let pool = sqlx::PgPool::connect(&db)
        .await
        .expect("db error");

    HttpServer::new(move || App::new()
        .app_data(web::Data::new(task_list::repo::TaskListRepository::new(pool.clone())))
        .app_data(web::Data::new(task::repo::TaskRepository::new(pool.clone())))
        .app_data(web::Data::new(user::repo::UserRepository::new(pool.clone())))
        .app_data(web::Data::new(Config::default().scope("access")))
        .wrap(Logger::default())
        .service(web::scope("/api")
            .route("/task-lists", web::get().to(get_task_lists))
            .route("/task-lists", web::post().to(create_task_list))
            .route("/task-lists/{id}", web::delete().to(delete_task_list))
            .route("/task-lists/{id}", web::get().to(get_task_list))
            .route("/task-lists/{id}", web::put().to(update_task_list))
            .route("/tasks", web::get().to(get_tasks))
            .route("/tasks", web::post().to(create_task))
            .route("/tasks/{id}", web::delete().to(delete_task))
            .route("/tasks/{id}", web::get().to(get_task))
            .route("/tasks/{id}", web::put().to(update_task))
            .route("/user", web::get().to(get_user))
        )
    )
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
