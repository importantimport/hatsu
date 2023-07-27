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
use serde::{Deserialize, Serialize};

use crate::{
  AppData,
  error::Error,
};

#[debug_handler]
pub async fn nodeinfo_2_0(
  _data: Data<AppData>,
) -> Result<Json<NodeInfo>, Error> {
  let nodeinfo = NodeInfo {
    version: "2.1".to_string(),
    software: NodeInfoSoftware {
      name: "Hatsu".to_string(),
      version: option_env!("CARGO_PKG_VERSION").unwrap().to_string(),
      repository: None,
      homepage: None,
    },
    protocols: vec!["activitypub".to_string()],
    usage: NodeInfoUsage {
      /// TODO
      users: Some(NodeInfoUsers {
        total: None,
        active_halfyear: None,
        active_month: None
      }),
      local_posts: None,
      local_comments: None
    },
    open_registrations: false
  };

  Ok(Json(nodeinfo))
}


#[debug_handler]
pub async fn nodeinfo_2_1(
  _data: Data<AppData>,
) -> Result<Json<NodeInfo>, Error> {
  let nodeinfo = NodeInfo {
    version: "2.1".to_string(),
    software: NodeInfoSoftware {
      name: "Hatsu".to_string(),
      version: option_env!("CARGO_PKG_VERSION").unwrap().to_string(),
      repository: Some("https://github.com/importantimport/hatsu".to_string()),
      homepage: Some("https://github.com/importantimport/hatsu".to_string()),
    },
    protocols: vec!["activitypub".to_string()],
    usage: NodeInfoUsage {
      /// TODO
      users: Some(NodeInfoUsers {
        total: None,
        active_halfyear: None,
        active_month: None
      }),
      local_posts: None,
      local_comments: None
    },
    open_registrations: false
  };

  Ok(Json(nodeinfo))
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
  pub local_posts: Option<i64>,
  pub local_comments: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct NodeInfoUsers {
  pub total: Option<i64>,
  pub active_halfyear: Option<i64>,
  pub active_month: Option<i64>,
}
