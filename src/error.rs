use std::fmt::{Display, Formatter};

use axum::{
    Json,
    response::{IntoResponse, Response},
    http::StatusCode,
};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    NotFound {
        kind: String,
        name: String
    },
    Anyhow(anyhow::Error)
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::NotFound { kind, name } => (StatusCode::NOT_FOUND, format!("Unable to find {} named {}", kind, name)),
            Self::Anyhow(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound { kind, name } => Display::fmt(&format!("Unable to find {} named {}", kind, name), f),
            Self::Anyhow(err) => Display::fmt(&err, f)
        }
    }
}

impl<T> From<T> for AppError
where
    T: Into<anyhow::Error>,
{
    fn from(t: T) -> Self {
        AppError::Anyhow(t.into())
    }
}
