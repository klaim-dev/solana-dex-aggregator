use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum AppError {
    #[error("not found")]
    NotFound,
    #[error("bad request {0}")]
    BadRequest(String),
    #[error("internal error")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, body) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "not found".to_string()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Internal(msg) => {
                tracing::error!("internal error: {msg}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal error".to_string(),
                )
            }
        };

        (status, body).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bad_request_displays_message() {
        let err = AppError::BadRequest("x too long".into());
        assert!(err.to_string().contains("x too long"));
    }

    #[test]
    fn not_found_display_message() {
        let err = AppError::NotFound;
        assert_eq!(err, AppError::NotFound);
        assert_eq!(err.to_string(), "not found");
    }

    #[test]
    fn internal_display_message() {
        let err = AppError::Internal("err".into());
        assert_eq!(err.to_string(), "internal error");
    }
}
