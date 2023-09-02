use activitypub_federation::kinds::collection::{OrderedCollectionType, OrderedCollectionPageType};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::AppError;

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
    prev: Option<Url>,
    // example: https://hatsu.local/u/example.com/followers?page=3
    next: Option<Url>,

    // example: https://hatsu.local/u/example.com/followers
    part_of: Option<Url>,

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
            first: Url::parse_with_params(&followers_id.to_string(), &[("page", "1")])?,
            total_items,
        })
    }
}
