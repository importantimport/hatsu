use activitypub_federation::kinds::collection::{OrderedCollectionType, OrderedCollectionPageType};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;

use crate::AppError;

pub fn generate_outbox_page_url(outbox_id: &Url, page: u64) -> Result<Url, AppError> {
    Ok(Url::parse_with_params(outbox_id.as_ref(), &[("page", page.to_string())])?)
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutboxPage {
    #[serde(rename = "type")]
    kind: OrderedCollectionPageType,
    // example: https://hatsu.local/u/example.com/outbox?page=2
    id: Url,

    // example: https://hatsu.local/u/example.com/outbox?page=1
    #[serde(skip_serializing_if = "Option::is_none")]
    prev: Option<Url>,
    // example: https://hatsu.local/u/example.com/outbox?page=3
    #[serde(skip_serializing_if = "Option::is_none")]
    next: Option<Url>,

    // example: https://hatsu.local/u/example.com/followers
    part_of: Url,

    // activities list (20 per page)
    ordered_items: Vec<Value>,
    // activities count
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

impl OutboxPage {
    pub fn new(outbox_id: Url, total_items: u64, ordered_items: Vec<Value>, total_pages: u64, page: u64) -> Result<Self, AppError> {
        Ok(Self {
            kind: OrderedCollectionPageType::OrderedCollectionPage,
            id: Url::parse_with_params(outbox_id.as_ref(), &[("page", page.to_string())])?,
            /// 如果当前页数大于 1，则提供上一页
            prev: match page {
                page if page > 1 => Some(generate_outbox_page_url(&outbox_id, page - 1)?),
                _ => None,
            },
            /// 如果当前页数小于总页数，则提供下一页
            next: match page {
                page if page < total_pages => Some(generate_outbox_page_url(&outbox_id, page + 1)?),
                _ => None,
            },
            part_of: outbox_id,
            ordered_items,
            total_items,
        })
    }
}
