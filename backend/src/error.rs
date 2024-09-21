use actix_web::{HttpResponse, ResponseError};
use actix_web::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError<'a> {
    #[error("Bad paginate count: {0}")]
    BadPaginate(i32),

    #[error("Bad parameter: {0}")]
    BadParam(&'a str),

    #[error("Unauthorized: {0}")]
    Unauthorized(&'a str),

    #[error("Internal server error")]
    ServerError,
}

impl ResponseError for ApiError<'_> {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::BadPaginate(_) => StatusCode::BAD_REQUEST,
            ApiError::BadParam(_) => StatusCode::BAD_REQUEST,
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiError::ServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}
