use hatsu_utils::AppError;
use serde::{Deserialize, Serialize};
use url::Url;
use utoipa::ToSchema;

mod collection;
mod collection_page;

pub use collection::Collection;
pub use collection_page::CollectionPage;

pub fn generate_collection_page_url(collection_id: &Url, page: u64) -> Result<Url, AppError> {
    Ok(Url::parse_with_params(collection_id.as_ref(), &[(
        "page",
        page.to_string(),
    )])?)
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[serde(untagged)]
pub enum CollectionOrPage {
    Collection(Collection),
    CollectionPage(CollectionPage),
}
