pub mod db {
    use libsql::Database;
    use serde_json::{Value, json};
    use crate::models::room::PersonalRoom;

    fn vector_to_string(strings: &[String]) -> String {
        json!(strings).to_string()
    }
    
    fn string_to_vector(string: String) -> Option<Vec<String>> {
        let json_value: Value = match serde_json::from_str(string.as_str()) {
            Ok(v) => v,
            Err(_) => return None,
        };

        let array: &Vec<Value> = json_value.as_array().expect("JSON value is not an array");
        let strings: Vec<String> = array.iter().map(|value| value.as_str().unwrap().to_string()).collect();
        Some(strings)
    }

    pub async fn insert(r: PersonalRoom, client: &Database, id: &str)-> Result<u64, libsql::Error> {
        let conn: libsql::Connection = client.connect().unwrap();
        conn.execute("INSERT INTO personal_rooms (id, name, description, members, created_at, media_uris) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            [id, &r.name, &r.description, &vector_to_string(&r.members.unwrap()), &r.created_at, &vector_to_string(&r.media_uris.unwrap())],
        ).await
    }
    
    pub async fn get_all(client: &Database) -> Option<Vec<PersonalRoom>> {
        let conn: libsql::Connection = match client.connect() {
            Ok(c) => c,
            Err(e) => {
                println!("error: {:?}", e);
                return None
            },
        };
        match conn.query("SELECT name, description, members, created_at, media_uris, id FROM personal_rooms", ()).await {
            Ok(mut rows) => {
                let mut users: Vec<PersonalRoom> = vec![];
                while let Some(row) = rows.next().await.unwrap() {
                    users.push(PersonalRoom {
                        name: row.get::<String>(0).unwrap_or_else(|_| "null".to_string()),
                        description: row.get::<String>(1).unwrap_or_else(|_| "null".to_string()),
                        members: string_to_vector(row.get::<String>(2).unwrap_or_else(|_| "null".to_string())),
                        created_at: row.get::<String>(3).unwrap_or_else(|_| "null".to_string()),
                        media_uris: string_to_vector(row.get::<String>(4).unwrap_or_else(|_| "null".to_string())),
                        id: row.get::<String>(5).unwrap_or_else(|_| "null".to_string()),
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
}