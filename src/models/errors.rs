use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse
};
use derive_more::{Display, Error};



#[derive(Debug, Display, Error)]
pub enum HttpError {
    #[display(fmt = "internal server error")]
    InternalError,
    #[display(fmt = "unauthorized error")]
    Unauthorized,
    #[display(fmt = "unauthenticated error")]
    Unauthenticated,
    #[display(fmt = "request timeout")]
    Timeout,
    #[display(fmt = "invalid user credentials")]
    InvalidCredentials,
    #[display(fmt = "Nats error")]
    NatsError,
}

impl error::ResponseError for HttpError{
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
        .insert_header(ContentType::json())
        .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            HttpError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            HttpError::Unauthorized => StatusCode::UNAUTHORIZED,
            HttpError::Unauthenticated => StatusCode::UNAUTHORIZED,
            HttpError::Timeout => StatusCode::REQUEST_TIMEOUT,
            HttpError::InvalidCredentials => StatusCode::BAD_REQUEST,
            HttpError::NatsError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}