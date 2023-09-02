use activitypub_federation::kinds::collection::{OrderedCollectionType, OrderedCollectionPageType};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::AppError;

pub fn generate_followers_page_url(followers_id: &Url, page: u64) -> Result<Url, AppError> {
    Ok(Url::parse_with_params(&followers_id.to_string(), &[("page", page.to_string())])?)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Followers {
    #[serde(rename = "type")]
    kind: OrderedCollectionType,
    // example: https://hatsu.local/u/example.com/followers
    id: Url,

    // example: https://hatsu.local/u/example.com/followers?page=1
    first: Url,

    // followers count
    total_items: u64
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FollowersPage {
    #[serde(rename = "type")]
    kind: OrderedCollectionPageType,
    // example: https://hatsu.local/u/example.com/followers?page=2
    id: Url,

    // example: https://hatsu.local/u/example.com/followers?page=1
    #[serde(skip_serializing_if = "Option::is_none")]
    prev: Option<Url>,
    // example: https://hatsu.local/u/example.com/followers?page=3
    #[serde(skip_serializing_if = "Option::is_none")]
    next: Option<Url>,

    // example: https://hatsu.local/u/example.com/followers
    part_of: Url,

    // followers url list (12 per page)
    ordered_items: Vec<Url>,
    // followers count
    total_items: u64
}

impl Followers {
    pub fn new(followers_id: Url, total_items: u64) -> Result<Self, AppError> {
        Ok(Self {
            kind: OrderedCollectionType::OrderedCollection,
            id: followers_id.clone(),
            first: generate_followers_page_url(&followers_id, 1)?,
            total_items,
        })
    }
}

impl FollowersPage {
    pub fn new(followers_id: Url, total_items: u64, ordered_items: Vec<Url>, total_pages: u64, page: u64) -> Result<Self, AppError> {
        Ok(Self {
            kind: OrderedCollectionPageType::OrderedCollectionPage,
            id: Url::parse_with_params(&followers_id.to_string(), &[("page", page.to_string())])?,
            /// 如果当前页数大于 1，则提供上一页
            prev: match page {
                page if page > 1 => Some(generate_followers_page_url(&followers_id, page - 1)?),
                _ => None,
            },
            /// 如果当前页数小于总页数，则提供下一页
            next: match page {
                page if page < total_pages => Some(generate_followers_page_url(&followers_id, page + 1)?),
                _ => None,
            },
            part_of: followers_id,
            ordered_items,
            total_items,
        })
    }
}
