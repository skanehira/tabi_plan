use axum::response::IntoResponse;
use axum::response::Response;
use reqwest::StatusCode;
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize)]
pub struct AppError {
    pub message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let message = serde_json::to_string(&self).expect("cannot serialize json");
        (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self {
            message: err.into().to_string(),
        }
    }
}
