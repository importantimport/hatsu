use std::fmt::{Display, Formatter};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::Value;
use tracing_error::SpanTrace;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema)]
pub struct AppError {
    /// An error message.
    pub error: String,
    /// A unique error ID.
    pub error_id: Uuid,
    /// Optional Additional error details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<Value>,
    #[serde(skip)]
    pub status: StatusCode,
    #[serde(skip)]
    pub context: SpanTrace,
}

impl AppError {
    #[must_use]
    pub fn new(error: String, error_details: Option<Value>, status: Option<StatusCode>) -> Self {
        Self {
            error,
            error_details,
            error_id: Uuid::new_v4(),
            status: status.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            context: SpanTrace::capture(),
        }
    }

    #[must_use]
    pub fn not_found(kind: &str, name: &str) -> Self {
        Self {
            error: format!("Unable to find {kind} named {name}"),
            error_details: None,
            error_id: Uuid::new_v4(),
            status: StatusCode::NOT_FOUND,
            context: SpanTrace::capture(),
        }
    }

    #[must_use]
    pub fn anyhow(error: &anyhow::Error) -> Self {
        Self::new(error.to_string(), None, None)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.status, Json(self)).into_response()
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self.error)?;
        self.context.fmt(f)?;
        Ok(())
    }
}

impl<T> From<T> for AppError
where
    T: Into<anyhow::Error>,
{
    fn from(t: T) -> Self {
        Self::anyhow(&t.into())
    }
}
