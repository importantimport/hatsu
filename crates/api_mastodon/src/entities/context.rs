use activitypub_federation::{config::Data, traits::Object};
use futures::future::TryJoinAll;
use hatsu_apub::objects::{ApubPost, Note};
use hatsu_db_schema::{post, prelude::Post};
use hatsu_utils::{AppData, AppError};
use sea_orm::{EntityTrait, ModelTrait};
use serde::{Deserialize, Serialize};
use url::Url;
use utoipa::ToSchema;

use crate::entities::Status;

/// <https://docs.joinmastodon.org/entities/Context/>
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Context {
    /// should always be empty vec
    pub ancestors: Vec<Status>,
    pub descendants: Vec<Status>,
}

impl Context {
    pub async fn find_by_id(post_id: &Url, data: &Data<AppData>) -> Result<Self, AppError> {
        match Post::find_by_id(post_id.to_string())
            .one(&data.conn)
            .await?
        {
            Some(post) => {
                Ok(Self {
                    ancestors: vec![],
                    // https://www.sea-ql.org/SeaORM/docs/relation/chained-relations/
                    descendants: post
                        .find_linked(post::SelfReferencingLink)
                        .all(&data.conn)
                        .await?
                        .into_iter()
                        .map(|post| async move {
                            let apub_post: ApubPost = post.clone().into();
                            let note: Note = apub_post.into_json(data).await?;
                            Status::from_json(note, data).await
                        })
                        .collect::<TryJoinAll<_>>()
                        .await?,
                })
            },
            None => Err(AppError::not_found("Record", post_id.as_ref())),
        }
    }
}
