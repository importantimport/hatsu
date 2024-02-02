use chrono::{DateTime, SecondsFormat, Utc};

use crate::AppError;

pub fn now() -> String {
    Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true)
}

pub fn parse(date: &str) -> Result<DateTime<Utc>, AppError> {
    Ok(DateTime::parse_from_rfc3339(date)?.with_timezone(&Utc))
}
