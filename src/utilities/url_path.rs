use std::str;

use url::Url;

use crate::error::Error;

pub fn absolutize_relative_url(url: String, domain: String) -> Result<String, Error> {
    if str::starts_with(&url, "https://") {
        Ok(url)
    } else {
        let origin = Url::parse(&format!("https://{}", domain))?;
        let absolute_url = origin.join(url.as_str())?.to_string();
        Ok(absolute_url)
    }
}
