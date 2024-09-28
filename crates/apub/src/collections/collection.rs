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
    kind: OrderedCollectionType,
    // example: https://hatsu.local/users/example.com/collection
    id: Url,

    // example: https://hatsu.local/users/example.com/collection?page=1
    first: Url,
    // example: https://hatsu.local/users/example.com/collection?page=64
    #[serde(skip_serializing_if = "Option::is_none")]
    last: Option<Url>,

    // collection count
    total_items: u64,
}

impl Collection {
    pub fn new(
        collection_id: &Url,
        total_items: u64,
        total_pages: Option<u64>,
    ) -> Result<Self, AppError> {
        Ok(Self {
            kind: OrderedCollectionType::OrderedCollection,
            id: collection_id.clone(),
            first: generate_collection_page_url(collection_id, 1)?,
            last: match total_pages {
                Some(total_pages) => Some(generate_collection_page_url(
                    collection_id,
                    match total_pages {
                        page if total_pages > 0 => page + 1,
                        _ => 1,
                    },
                )?),
                None => None,
            },
            total_items,
        })
    }
}
