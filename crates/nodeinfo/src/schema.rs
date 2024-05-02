use activitypub_federation::config::Data;
use hatsu_db_schema::{
    post,
    prelude::{Post, User},
    user,
};
use hatsu_utils::{AppData, AppError};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// `NodeInfo` schema.
///
/// <https://nodeinfo.diaspora.software/schema.html>
#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct NodeInfo {
    pub version: String,
    pub software: NodeInfoSoftware,
    pub protocols: Vec<String>,
    pub services: NodeInfoServices,
    pub open_registrations: bool,
    pub usage: NodeInfoUsage,
    pub metadata: NodeInfoMetadata,
}

impl NodeInfo {
    pub async fn v2_0(data: &Data<AppData>) -> Result<Self, AppError> {
        Ok(Self {
            version: String::from("2.0"),
            software: NodeInfoSoftware {
                name: String::from("hatsu"),
                version: String::from(env!("CARGO_PKG_VERSION")),
                repository: None,
                homepage: None,
            },
            protocols: vec![String::from("activitypub")],
            services: NodeInfoServices::new(),
            open_registrations: false,
            usage: NodeInfoUsage::new(data).await?,
            metadata: NodeInfoMetadata::new(data),
        })
    }

    pub async fn v2_1(data: &Data<AppData>) -> Result<Self, AppError> {
        Ok(Self {
            version: String::from("2.1"),
            software: NodeInfoSoftware {
                name: String::from("hatsu"),
                version: String::from(env!("CARGO_PKG_VERSION")),
                repository: Some(String::from(env!("CARGO_PKG_REPOSITORY"))),
                homepage: Some(String::from(env!("CARGO_PKG_HOMEPAGE"))),
            },
            ..Self::v2_0(data).await?
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct NodeInfoSoftware {
    pub name: String,
    pub version: String,
    /// Only available for `NodeInfo` 2.1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    /// Only available for `NodeInfo` 2.1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct NodeInfoServices {
    pub inbound: Vec<String>,
    pub outbound: Vec<String>,
}

impl NodeInfoServices {
    fn new() -> Self {
        Self {
            inbound: vec![],
            outbound: vec![],
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct NodeInfoUsage {
    pub users: Option<NodeInfoUsers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_posts: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_comments: Option<u64>,
}

impl NodeInfoUsage {
    async fn new(data: &Data<AppData>) -> Result<Self, AppError> {
        Ok(Self {
            users: Some(NodeInfoUsers::new(data).await?),
            local_posts: Some(
                Post::find()
                    .filter(post::Column::Local.eq(true))
                    .count(&data.conn)
                    .await?,
            ),
            local_comments: None,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct NodeInfoUsers {
    pub total: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_halfyear: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_month: Option<u64>,
}

impl NodeInfoUsers {
    async fn new(data: &Data<AppData>) -> Result<Self, AppError> {
        Ok(Self {
            total: User::find()
                .filter(user::Column::Local.eq(true))
                .count(&data.conn)
                .await?,
            active_halfyear: None,
            active_month: None,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct NodeInfoMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_description: Option<String>,
}

impl NodeInfoMetadata {
    fn new(data: &Data<AppData>) -> Self {
        Self {
            node_name: data.env.hatsu_node_name.clone(),
            node_description: data.env.hatsu_node_description.clone(),
        }
    }
}
