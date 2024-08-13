pub mod jwt {
    use std::{env, time::{Duration, SystemTime}};

    use crate::models::auth::{Claims, UserModel};

    pub fn encode(user: UserModel) -> String {
        // create an exp time
        let exp_time = std::time::SystemTime::now()
            .checked_add(Duration::from_secs(60))
            .unwrap()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let c = Claims{
            custom_claim: user,
            iss: env::var("JWT_ISSUER").expect("JWT SECRET was not set"),
            sub: env::var("JWT_SUBJECT").expect("JWT SECRET was not set"),
            aud: env::var("JWT_AUDIENCE").expect("JWT SECRET was not set"),
            exp: exp_time,
        };

        // create a header
        let header = jsonwebtoken::Header::default();
        let secret = env::var("JWT_SECRET").expect("JWT SECRET was not set");
        let key = jsonwebtoken::EncodingKey::from_secret(secret.as_bytes());
        let token = jsonwebtoken::encode(&header, &c, &key).unwrap();
        token
    }

    pub fn decode(token: &str)->  bool{
        let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
        let secret = env::var("JWT_SECRET").expect("JWT SECRET was not set");
        validation.set_audience(&vec![env::var("JWT_AUDIENCE").expect("JWT SECRET was not set")]);
        let key = jsonwebtoken::DecodingKey::from_secret(secret.as_bytes());
         match jsonwebtoken::decode::<Claims>(token, &key, &validation) {
            Ok(token) => {
                return token.claims.iss == env::var("JWT_ISSUER").expect("JWT SECRET was not set")
            },
            Err(e) => {
                println!("{:?}", e);
                return false
            }
         };
    }
}