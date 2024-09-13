pub mod db {
    use libsql::Database;
    use crate::models::user::UserModel;

    pub async fn insert(user: UserModel, client: &Database, id: &str)-> Result<u64, libsql::Error> {
        let conn: libsql::Connection = client.connect().unwrap();
        conn.execute(
            "INSERT INTO users (id, firstname, lastname, email, username, profile_uri, status, password, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            [id, &user.firstname, &user.lastname, &user.email, &user.username, &user.profile_uri, &user.status, &user.password, &user.created_at],
        ).await
    }

    pub async fn get(id: &str, client: &Database) -> Option<UserModel> {
        let conn: libsql::Connection = client.connect().unwrap();
        match conn.query("SELECT firstname, lastname, email, username, profile_uri, status, password, created_at, id FROM users WHERE id = ?1", [id]).await {
            Ok(mut rows) => {
                let row = rows.next().await.unwrap().unwrap(); 
                Some(UserModel {
                    firstname: row.get::<String>(0).unwrap(),
                    lastname: row.get::<String>(1).unwrap(),
                    email: row.get::<String>(2).unwrap(),
                    username: row.get::<String>(3).unwrap(),
                    profile_uri: row.get::<String>(4).unwrap(),
                    status: row.get::<String>(5).unwrap(),
                    password: row.get::<String>(6).unwrap(),
                    created_at: row.get::<String>(7).unwrap(),
                    id: row.get::<String>(8).unwrap(),
                })
            },
            Err(e) => {
                eprint!("could not retrieve user from the database. error: {:?}", e);
                None
            }
        }
    }

    pub async fn get_by_email(email: &str, client: &Database) -> Option<UserModel> {
        let conn: libsql::Connection = client.connect().unwrap();
        match conn.query("SELECT firstname, lastname, email, username, profile_uri, status, password, created_at, id FROM users WHERE email = ?1", [email]).await {
            Ok(mut rows) => {
                let row = rows.next().await.unwrap();
                if let Some(row) = row {
                    return Some(UserModel {
                        firstname: row.get::<String>(0).unwrap(),
                        lastname: row.get::<String>(1).unwrap(),
                        email: row.get::<String>(2).unwrap(),
                        username: row.get::<String>(3).unwrap(),
                        profile_uri: row.get::<String>(4).unwrap(),
                        status: row.get::<String>(5).unwrap(),
                        password: row.get::<String>(6).unwrap(),
                        created_at: row.get::<String>(7).unwrap(),
                        id: row.get::<String>(8).unwrap(),
                    })
                } else {None}
                
            },
            Err(e) => {
                eprint!("could not retrieve user from database. error: {:?}", e);
                None
            }
        }
    }

    pub async fn get_all(client: &Database) -> Option<Vec<UserModel>> {
        let conn: libsql::Connection = client.connect().unwrap();
        match conn.query("SELECT firstname, lastname, email, username, profile_uri, status, password, created_at, id FROM users", ()).await {
            Ok(mut rows) => {
                let mut users: Vec<UserModel> = vec![];
                while let Some(row) = rows.next().await.unwrap() {
                    users.push(UserModel {
                        firstname: row.get::<String>(0).unwrap(),
                        lastname: row.get::<String>(1).unwrap(),
                        email: row.get::<String>(2).unwrap(),
                        username: row.get::<String>(3).unwrap(),
                        profile_uri: row.get::<String>(4).unwrap(),
                        status: row.get::<String>(5).unwrap(),
                        password: row.get::<String>(6).unwrap(),
                        created_at: row.get::<String>(7).unwrap(),
                        id: row.get::<String>(8).unwrap(),
                    });
                };
                Some(users)
            },
            Err(e) => {
                eprint!("could not retrieve all items from the database. error: {:?}", e);
                None
            }
        }
    }

    pub async fn delete(id: &str, client: &Database) -> Result<u64, libsql::Error> {
        let conn: libsql::Connection = client.connect().unwrap();
        conn.execute("DELETE FROM users WHERE id = ?1",
            [id],
        ).await
    }
}