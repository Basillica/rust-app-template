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

    #[allow(dead_code)]
    pub async fn get_by_email(email: String, pool: &PgPool) -> Result<UserModel, sqlx::Error> {
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


#[cfg(test)]
mod db_test {
    use std::env;
    use dotenv::dotenv;
    use sqlx::{postgres::PgPoolOptions, PgPool};
    use crate::models::auth::UserModel;
    use super::users;

    
    #[actix_web::test]
    async fn test_user_insert() {
        dotenv().ok();

        let pool = create_db_pool().await;
        let user = UserModel{
            firstname: String::from("tonie"),
            lastname: String::from("etienne"),
            email: String::from("tonie.etienne@example.com"),
            password: String::from("someboringpassword"),
            id: String::from("someboringid"),
        };
        let id = "someboringid";

        // test insert and get
        users::insert(user, &pool, id).await.unwrap();
        let user = match users::get(id.to_string(), &pool).await {
            Ok(u) => u,
            Err(_) => UserModel::default(),
        };
        assert_eq!(user.id, id);

        // test get user by email
        let user = match users::get_by_email(user.email, &pool).await {
            Ok(u) => u,
            Err(_) => UserModel::default(),
        };
        assert_eq!(user.id, id);

        // test delete user
        let result = users::delete(id.to_string(), &pool).await.unwrap();
        assert_eq!(result.rows_affected(), 1)

    }

    async fn create_db_pool() -> PgPool {
        let db_conn_str = env::var("DATABASE_CONNECTION_STRING").expect("the database connection string was not set");
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_conn_str)
            .await
            .unwrap()
    }
}