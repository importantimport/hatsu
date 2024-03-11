pub mod activities;
pub mod actors;
pub mod collections;
pub mod links;
pub mod objects;

// #[cfg(test)]
pub mod tests {
    use std::{fs::File, io::BufReader};

    use activitypub_federation::protocol::context::WithContext;
    use hatsu_utils::AppError;
    use serde::de::DeserializeOwned;

    pub fn test_asset<T: DeserializeOwned>(path: &str) -> Result<WithContext<T>, AppError> {
        let asset = File::open(path)?;
        let reader = BufReader::new(asset);
        Ok(serde_json::from_reader(reader)?)
    }
}
