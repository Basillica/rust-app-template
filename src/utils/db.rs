pub mod users {
    use sqlx::{postgres::{PgQueryResult, PgRow}, PgPool};

    use crate::models::auth::UserModel;

    pub async fn insert(user: UserModel, pool: &PgPool, id: &str)-> Result<PgRow, sqlx::Error> {
        sqlx::query(
            "INSERT INTO users (id, firstname, lastname, password, email) VALUES ($1, $2, $3, $4, $5) RETURNING id"
        )
            .bind(id)
            .bind(user.firstname)
            .bind(user.lastname)
            .bind(user.password)
            .bind(user.email)
            
            .fetch_one(pool)
            .await
    }

    pub async fn get(id: String, pool: &PgPool) -> Result<UserModel, sqlx::Error> {
       sqlx::query_as(
            "SELECT firstname, lastname, email, password, id FROM users WHERE id = $1"
        ).bind(id)
        .fetch_one(pool)
        .await
    }

    pub async fn getByEmail(email: String, pool: &PgPool) -> Result<UserModel, sqlx::Error> {
        sqlx::query_as(
             "SELECT firstname, lastname, email, password, id FROM users WHERE email = $1"
         ).bind(email)
         .fetch_one(pool)
         .await
     }

    pub async fn delete(id: String, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query(
            "DELETE FROM users WHERE id = $1"
        ).bind(id)
        .execute(pool)
        .await
    }

    pub async fn getall(pool: &PgPool) -> Result<Vec<UserModel>, sqlx::Error> {
        sqlx::query_as(
            "SELECT firstname, lastname, email, password, id FROM users ORDER BY id"
        ).fetch_all(pool)
        .await
    }
}