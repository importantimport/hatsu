// https://nodeinfo.diaspora.software/protocol.html
// https://github.com/LemmyNet/lemmy/blob/main/crates/routes/src/nodeinfo.rs

use activitypub_federation::config::Data;
use axum::{debug_handler, routing::get, Json, Router};
use hatsu_db_schema::{prelude::User, user};
use hatsu_utils::{AppData, AppError};
use sea_orm::{ActiveEnum, ActiveModelBehavior, ColumnTrait, EntityTrait, Iden, PaginatorTrait, QueryFilter};
use serde::{Deserialize, Serialize};

async fn nodeinfo_usage(data: Data<AppData>) -> Result<NodeInfoUsage, AppError> {
    Ok(NodeInfoUsage {
        users: Some(NodeInfoUsers {
            total: User::find()
                .filter(user::Column::Local.eq(true))
                .count(&data.conn)
                .await?,
            // TODO
            active_halfyear: None,
            active_month: None,
        }),
        // TODO
        local_posts: None,
        local_comments: None,
    })
}

/// https://github.com/jhass/nodeinfo/blob/main/schemas/2.0/schema.json
#[debug_handler]
pub async fn nodeinfo_2_0(data: Data<AppData>) -> Result<Json<NodeInfo>, AppError> {
    Ok(Json(NodeInfo {
        version: "2.0".to_string(),
        software: NodeInfoSoftware {
            name: "hatsu".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            repository: None,
            homepage: None,
        },
        protocols: vec!["activitypub".to_string()],
        services: NodeInfoServices {
            inbound: vec![],
            outbound: vec![],
        },
        open_registrations: false,
        usage: nodeinfo_usage(data).await?,
        metadata: NodeInfoMetadata {},
    }))
}

/// https://github.com/jhass/nodeinfo/blob/main/schemas/2.1/schema.json
#[debug_handler]
pub async fn nodeinfo_2_1(data: Data<AppData>) -> Result<Json<NodeInfo>, AppError> {
    Ok(Json(NodeInfo {
        version: "2.1".to_string(),
        software: NodeInfoSoftware {
            name: "hatsu".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            repository: Some("https://github.com/importantimport/hatsu".to_string()),
            homepage: Some("https://github.com/importantimport/hatsu".to_string()),
        },
        protocols: vec!["activitypub".to_string()],
        services: NodeInfoServices {
            inbound: vec![],
            outbound: vec![],
        },
        open_registrations: false,
        usage: nodeinfo_usage(data).await?,
        metadata: NodeInfoMetadata {},
    }))
}

pub fn handler() -> Router {
    Router::new()
        .route("/nodeinfo/2.0", get(nodeinfo_2_0))
        .route("/nodeinfo/2.0.json", get(nodeinfo_2_0))
        .route("/nodeinfo/2.1", get(nodeinfo_2_1))
        .route("/nodeinfo/2.1.json", get(nodeinfo_2_1))
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct NodeInfo {
    pub version: String,
    pub software: NodeInfoSoftware,
    pub protocols: Vec<String>,
    pub services: NodeInfoServices,
    pub open_registrations: bool,
    pub usage: NodeInfoUsage,
    pub metadata: NodeInfoMetadata,
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
#[serde(default)]
pub struct NodeInfoServices {
    pub inbound: Vec<String>,
    pub outbound: Vec<String>,
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

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct NodeInfoMetadata {}
