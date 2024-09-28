use activitypub_federation::kinds::collection::OrderedCollectionPageType;
use hatsu_utils::AppError;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::collections::generate_collection_page_url;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionPage<T> {
    #[serde(rename = "type")]
    kind: OrderedCollectionPageType,
    // example: https://hatsu.local/users/example.com/collection?page=2
    id: Url,
    // example: https://hatsu.local/users/example.com/collection?page=1
    #[serde(skip_serializing_if = "Option::is_none")]
    prev: Option<Url>,
    // example: https://hatsu.local/users/example.com/collection?page=3
    #[serde(skip_serializing_if = "Option::is_none")]
    next: Option<Url>,
    // example: https://hatsu.local/users/example.com/collection
    part_of: Url,
    // collection item list
    ordered_items: Vec<T>,
    // collection count
    total_items: u64,
}

impl<T> CollectionPage<T> {
    pub fn new(
        collection_id: Url,
        total_items: u64,
        ordered_items: Vec<T>,
        total_pages: u64,
        page: u64,
    ) -> Result<Self, AppError> {
        Ok(Self {
            kind: OrderedCollectionPageType::OrderedCollectionPage,
            id: Url::parse_with_params(collection_id.as_ref(), &[("page", page.to_string())])?,
            // 如果当前页数大于 1，则提供上一页
            prev: match page {
                page if page > 1 => Some(generate_collection_page_url(&collection_id, page - 1)?),
                _ => None,
            },
            // 如果当前页数小于总页数，则提供下一页
            next: match page {
                page if page < total_pages =>
                    Some(generate_collection_page_url(&collection_id, page + 1)?),
                _ => None,
            },
            part_of: collection_id,
            ordered_items,
            total_items,
        })
    }
}
