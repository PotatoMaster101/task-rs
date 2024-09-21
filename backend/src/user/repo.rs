use uuid::{ContextV7, Timestamp};
use crate::repo::*;
use crate::user::*;
use crate::user::requests::*;

const TABLE_NAME: &str = r#""app"."user""#;

#[derive(Clone, Debug)]
pub struct UserRepository {
    pool: sqlx::PgPool
}

impl UserRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

impl CanCreate<User, CreateUserRequest> for UserRepository {
    async fn create(&self, item: &CreateUserRequest) -> sqlx::Result<User> {
        let sql = format!(r#"INSERT INTO {} VALUES ($1, $2, $3, $4, $5) RETURNING *"#, TABLE_NAME);
        let now = Utc::now();
        sqlx::query_as::<_, User>(&sql)
            .bind(Uuid::new_v7(Timestamp::now(ContextV7::new())))
            .bind(now)
            .bind(now)
            .bind(&item.email)
            .bind(&item.auth0_id)
            .fetch_one(&self.pool)
            .await
    }
}

impl CanGet<User, String> for UserRepository {
    async fn get(&self, item: &String) -> sqlx::Result<User> {
        let sql = format!(r#"SELECT * FROM {} WHERE "auth0_id" = $1"#, TABLE_NAME);
        sqlx::query_as::<_, User>(&sql)
            .bind(item)
            .fetch_one(&self.pool)
            .await
    }
}

impl CanGetOrCreate<User, CreateUserRequest> for UserRepository {
    async fn get_or_create(&self, item: &CreateUserRequest) -> sqlx::Result<User> {
        let user = self.get(&item.auth0_id).await;
        if user.is_ok() {
            user
        } else {
            self.create(item).await
        }
    }
}
