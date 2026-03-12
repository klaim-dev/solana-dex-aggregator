use axum::{http::{StatusCode}, response::IntoResponse};

pub enum AppError {
    NotFound,
    BadRequest{input: String},
    Internal{input: String},
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = get_status(&self);
        let message = get_message(&self);
        (status, message.to_string()).into_response()
    }
}

fn get_status(input_error: &AppError) -> StatusCode {
    match input_error{
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::BadRequest { .. } => StatusCode::BAD_REQUEST,
            AppError::Internal { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
fn get_message(input_error: &AppError) -> &str {
    match input_error {
    AppError::NotFound => "not found",
    AppError::BadRequest { input } => input,
    AppError::Internal { .. } => "internal error",
}
}
