use activitypub_federation::kinds::collection::OrderedCollectionType;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::AppError;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Outbox {
  #[serde(rename = "type")]
  kind: OrderedCollectionType,
  id: Url,
  ordered_items: Vec<()>,
  total_items: u64
}

/// TODO: items (contents)
impl Outbox {
  pub fn new(outbox_id: Url) -> Result<Self, AppError> {
    Ok(Self {
      kind: OrderedCollectionType::OrderedCollection,
      id: outbox_id,
      ordered_items: vec![],
      total_items: 0,
    })
  }
}
