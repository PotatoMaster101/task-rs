use chrono::Utc;
use uuid::Uuid;
use crate::page::*;
use crate::repo::*;
use crate::task::{*, requests::*};

const TABLE_NAME: &str = r#""app"."task""#;

#[derive(Clone, Debug)]
pub struct TaskRepository {
    pool: sqlx::PgPool
}

impl TaskRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

impl CanCreate<Task, CreateTaskRequest> for TaskRepository {
    async fn create(&self, item: &CreateTaskRequest) -> sqlx::Result<Task> {
        let sql = format!(r#"INSERT INTO {} VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *"#, TABLE_NAME);
        let now = Utc::now();
        sqlx::query_as::<_, Task>(&sql)
            .bind(Uuid::new_v7(Timestamp::now(ContextV7::new())))
            .bind(item.list_id)
            .bind(item.user_id)
            .bind(now)
            .bind(now)
            .bind(&item.title)
            .bind(&item.description)
            .bind(item.done)
            .fetch_one(&self.pool)
            .await
    }
}

impl CanDelete<Task, Uuid> for TaskRepository {
    async fn delete(&self, item: &Uuid) -> sqlx::Result<Task> {
        let sql = format!(r#"DELETE FROM {} WHERE "id" = $1 RETURNING *"#, TABLE_NAME);
        sqlx::query_as::<_, Task>(&sql)
            .bind(item)
            .fetch_one(&self.pool)
            .await
    }
}

impl CanGet<Task, Uuid> for TaskRepository {
    async fn get(&self, item: &Uuid) -> sqlx::Result<Task> {
        let sql = format!(r#"SELECT * FROM {} WHERE "id" = $1"#, TABLE_NAME);
        sqlx::query_as::<_, Task>(&sql)
            .bind(item)
            .fetch_one(&self.pool)
            .await
    }
}

impl CanPaginate<Task, TaskPage> for TaskRepository {
    async fn paginate(&self, page: &TaskPage) -> sqlx::Result<Vec<Task>> {
        if page.last == Uuid::default() {
            let sql = format!(r#"SELECT * FROM {} WHERE "list_id" = $1 ORDER BY "id" LIMIT $2"#, TABLE_NAME);
            return sqlx::query_as::<_, Task>(&sql)
                .bind(page.list_id)
                .bind(page.count)
                .fetch_all(&self.pool)
                .await;
        }

        let sql = format!(r#"SELECT * FROM {} WHERE "list_id" = $1 AND "id" > $2 ORDER BY "id" LIMIT $3"#, TABLE_NAME);
        sqlx::query_as::<_, Task>(&sql)
            .bind(page.list_id)
            .bind(page.last)
            .bind(page.count)
            .fetch_all(&self.pool)
            .await
    }
}

impl CanUpdate<Task, UpdateTaskRequest> for TaskRepository {
    async fn update(&self, id: &Uuid, item: &UpdateTaskRequest) -> sqlx::Result<Task> {
        let sql = format!(r#"UPDATE {} SET "title" = $1, "description" = $2, "done" = $3, "updated_at" = $4 WHERE "id" = $5 RETURNING *"#, TABLE_NAME);
        sqlx::query_as::<_, Task>(&sql)
            .bind(&item.title)
            .bind(&item.description)
            .bind(item.done)
            .bind(Utc::now())
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }
}
