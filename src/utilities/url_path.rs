use std::str;

use url::Url;

use crate::AppError;

pub fn absolutize_relative_url(url: String, domain: String) -> Result<String, AppError> {
    if str::starts_with(&url, "https://") {
        Ok(url)
    } else {
        let origin = Url::parse(&format!("https://{}", domain))?;
        let absolute_url = origin.join(url.as_str())?.to_string();
        Ok(absolute_url)
    }
}

pub fn remove_https(url: String) -> String {
    if str::starts_with(&url, "https://") {
        let url_without_https = url.trim_start_matches("https://").to_string();
        url_without_https
    } else {
        url
    }
}
