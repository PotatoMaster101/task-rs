use chrono::Utc;
use uuid::{ContextV7, Timestamp, Uuid};
use crate::page::*;
use crate::repo::*;
use crate::task_list::{*, requests::*};

const TABLE_NAME: &str = r#""app"."task_list""#;

#[derive(Clone, Debug)]
pub struct TaskListRepository {
    pool: sqlx::PgPool
}

impl TaskListRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

impl CanCreate<TaskList, CreateTaskListRequest> for TaskListRepository {
    async fn create(&self, item: &CreateTaskListRequest) -> sqlx::Result<TaskList> {
        let sql = format!(r#"INSERT INTO {} VALUES ($1, $2, $3, $4, $5) RETURNING *"#, TABLE_NAME);
        let now = Utc::now();
        sqlx::query_as::<_, TaskList>(&sql)
            .bind(Uuid::new_v7(Timestamp::now(ContextV7::new())))
            .bind(item.user_id)
            .bind(now)
            .bind(now)
            .bind(&item.title)
            .fetch_one(&self.pool)
            .await
    }
}

impl CanDelete<TaskList, Uuid> for TaskListRepository {
    async fn delete(&self, item: &Uuid) -> sqlx::Result<TaskList> {
        let sql = format!(r#"DELETE FROM {} WHERE "id" = $1 RETURNING *"#, TABLE_NAME);
        sqlx::query_as::<_, TaskList>(&sql)
            .bind(item)
            .fetch_one(&self.pool)
            .await
    }
}

impl CanGet<TaskList, Uuid> for TaskListRepository {
    async fn get(&self, item: &Uuid) -> sqlx::Result<TaskList> {
        let sql = format!(r#"SELECT * FROM {} WHERE "id" = $1"#, TABLE_NAME);
        sqlx::query_as::<_, TaskList>(&sql)
            .bind(item)
            .fetch_one(&self.pool)
            .await
    }
}

impl CanPaginate<TaskList, Page> for TaskListRepository {
    async fn paginate(&self, page: &Page) -> sqlx::Result<Vec<TaskList>> {
        if page.last == Uuid::default() {
            let sql = format!(r#"SELECT * FROM {} ORDER BY "id" LIMIT $1"#, TABLE_NAME);
            return sqlx::query_as::<_, TaskList>(&sql)
                .bind(page.count)
                .fetch_all(&self.pool)
                .await;
        }

        let sql = format!(r#"SELECT * FROM {} WHERE "id" > $1 ORDER BY "id" LIMIT $2"#, TABLE_NAME);
        sqlx::query_as::<_, TaskList>(&sql)
            .bind(page.last)
            .bind(page.count)
            .fetch_all(&self.pool)
            .await
    }
}

impl CanUpdate<TaskList, UpdateTaskListRequest> for TaskListRepository {
    async fn update(&self, id: &Uuid, item: &UpdateTaskListRequest) -> sqlx::Result<TaskList> {
        let sql = format!(r#"UPDATE {} SET "title" = $1, "updated_at" = $2 WHERE "id" = $3 RETURNING *"#, TABLE_NAME);
        sqlx::query_as::<_, TaskList>(&sql)
            .bind(&item.title)
            .bind(Utc::now())
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }
}
