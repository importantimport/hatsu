use url::Url;

use crate::error::Error;

pub fn absolutize_relative_url(url: Url, domain: String) -> Result<Url, Error> {
    if url.scheme() == "https" {
        Ok(url)
    } else {
        let origin = Url::parse(&format!("https://{}", domain))?;
        let absolute_url = origin.join(url.as_str())?;
        Ok(absolute_url)
    }
}
