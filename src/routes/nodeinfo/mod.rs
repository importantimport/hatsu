// https://nodeinfo.diaspora.software/protocol.html
// https://github.com/LemmyNet/lemmy/blob/main/crates/routes/src/nodeinfo.rs

use activitypub_federation::config::Data;
use axum::{
  debug_handler,
  body::Body,
  routing::get,
  Json,
  Router,
};
use sea_orm::*;
use serde::{Deserialize, Serialize};

use crate::{
  AppData,
  entities::{prelude::*, *},
  error::Error,
};

async fn nodeinfo_usage(
  data: Data<AppData>,
) -> Result<NodeInfoUsage, Error> {
  Ok(NodeInfoUsage {
    users: Some(NodeInfoUsers {
      total: User::find()
        .filter(user::Column::Local.eq(true))
        .count(&data.conn)
        .await?,
      /// TODO
      active_halfyear: None,
      active_month: None
    }),
    /// TODO
    local_posts: None,
    local_comments: None
  })
}

#[debug_handler]
pub async fn nodeinfo_2_0(
  data: Data<AppData>,
) -> Result<Json<NodeInfo>, Error> {
  Ok(Json(NodeInfo {
    version: "2.0".to_string(),
    software: NodeInfoSoftware {
      name: "Hatsu".to_string(),
      version: option_env!("CARGO_PKG_VERSION").unwrap().to_string(),
      repository: None,
      homepage: None,
    },
    protocols: vec!["activitypub".to_string()],
    usage: nodeinfo_usage(data).await?,
    open_registrations: false
  }))
}


#[debug_handler]
pub async fn nodeinfo_2_1(
  data: Data<AppData>,
) -> Result<Json<NodeInfo>, Error> {
  Ok(Json(NodeInfo {
    version: "2.1".to_string(),
    software: NodeInfoSoftware {
      name: "Hatsu".to_string(),
      version: option_env!("CARGO_PKG_VERSION").unwrap().to_string(),
      repository: Some("https://github.com/importantimport/hatsu".to_string()),
      homepage: Some("https://github.com/importantimport/hatsu".to_string()),
    },
    protocols: vec!["activitypub".to_string()],
    usage: nodeinfo_usage(data).await?,
    open_registrations: false
  }))
}

pub fn init() -> Router<(), Body> {
    let router = Router::new()
        .route("/nodeinfo/2.0.json", get(nodeinfo_2_0))
        .route("/nodeinfo/2.1.json", get(nodeinfo_2_1));

    router
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct NodeInfo {
  pub version: String,
  pub software: NodeInfoSoftware,
  pub protocols: Vec<String>,
  pub usage: NodeInfoUsage,
  pub open_registrations: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct NodeInfoSoftware {
  pub name: String,
  pub version: String,
  /// Only available for NodeInfo 2.1
  #[serde(skip_serializing_if = "Option::is_none")]
  pub repository: Option<String>,
  /// Only available for NodeInfo 2.1
  #[serde(skip_serializing_if = "Option::is_none")]
  pub homepage: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct NodeInfoUsage {
  pub users: Option<NodeInfoUsers>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub local_posts: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub local_comments: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct NodeInfoUsers {
  pub total: u64,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub active_halfyear: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub active_month: Option<u64>,
}
