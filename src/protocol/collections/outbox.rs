use activitypub_federation::kinds::collection::OrderedCollectionType;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::AppError;

pub fn generate_outbox_page_url(outbox_id: &Url, page: u64) -> Result<Url, AppError> {
    Ok(Url::parse_with_params(&outbox_id.to_string(), &[("page", page.to_string())])?)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Outbox {
    #[serde(rename = "type")]
    kind: OrderedCollectionType,
    // example: https://hatsu.local/u/example.com/outbox
    id: Url,
    // example: https://hatsu.local/u/example.com/outbox?page=1
    first: Url,
    // example: https://hatsu.local/u/example.com/outbox?page=64
    last: Url,

    total_items: u64
}

impl Outbox {
    pub fn new(outbox_id: Url, total_items: u64, total_pages: u64) -> Result<Self, AppError> {
        Ok(Self {
            kind: OrderedCollectionType::OrderedCollection,
            id: outbox_id.clone(),
            first: generate_outbox_page_url(&outbox_id, 1)?,
            last: generate_outbox_page_url(
                &outbox_id,
                // TODO: 测试效果
                match total_pages {
                    page if total_pages > 0 => page + 1,
                    _ => 1,
                }
            )?,
            total_items,
        })
    }
}
