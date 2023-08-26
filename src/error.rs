use std::fmt::{Display, Formatter};

use axum::{
    Json,
    response::{IntoResponse, Response},
    http::StatusCode,
};
use serde_json::json;

#[derive(Debug)]
pub struct AppError(pub(crate) anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": self.0.to_string() }))
        )
            .into_response()
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl<T> From<T> for AppError
where
    T: Into<anyhow::Error>,
{
    fn from(t: T) -> Self {
        AppError(t.into())
    }
}
