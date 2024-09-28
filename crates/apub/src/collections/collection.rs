use activitypub_federation::kinds::collection::OrderedCollectionType;
use hatsu_utils::AppError;
use serde::{Deserialize, Serialize};
use url::Url;
use utoipa::ToSchema;

use crate::collections::generate_collection_page_url;

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    #[serde(rename = "type")]
    pub kind: OrderedCollectionType,
    // example: https://hatsu.local/users/example.com/collection
    pub id: Url,
    // example: https://hatsu.local/users/example.com/collection?page=1
    pub first: Url,
    // example: https://hatsu.local/users/example.com/collection?page=64
    pub last: Url,
    // collection count
    pub total_items: u64,
}

impl Collection {
    pub fn new(collection_id: &Url, total_items: u64, total_pages: u64) -> Result<Self, AppError> {
        Ok(Self {
            kind: OrderedCollectionType::OrderedCollection,
            id: collection_id.clone(),
            first: generate_collection_page_url(collection_id, 1)?,
            last: generate_collection_page_url(collection_id, match total_pages {
                page if page > 0 => page,
                _ => 1,
            })?,
            total_items,
        })
    }
}
