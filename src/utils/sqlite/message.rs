pub mod db {
    use libsql::Database;
    use serde_json::{Value, json};
    use crate::models::message::PersonalMessage;


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

    pub async fn insert(m: PersonalMessage, client: &Database) -> Result<u64, libsql::Error>{
        let conn: libsql::Connection = client.connect().unwrap();
        conn.execute("INSERT INTO personal_messages (id, room_id, user_id, message, media_uris, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            [m.id, m.room_id, m.user_id, m.message, vector_to_string(&m.media_uris.unwrap()), m.created_at],
        ).await
    }
}