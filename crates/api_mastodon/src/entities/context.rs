use activitypub_federation::{config::Data, traits::Object};
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
        match Post::find_by_id(&post_id.to_string())
            .one(&data.conn)
            .await?
        {
            Some(post) => {
                // https://www.sea-ql.org/SeaORM/docs/relation/chained-relations/
                let handles = post
                    .find_linked(post::SelfReferencingLink)
                    .all(&data.conn)
                    .await?
                    .into_iter()
                    .map(|post| async move {
                        let apub_post: ApubPost = post.clone().into();
                        // TODO: remove unwrap
                        let note: Note = apub_post.into_json(data).await.unwrap();

                        Status::from_json(note, data).await.unwrap()
                    })
                    .collect::<Vec<_>>();

                let descendants: Vec<Status> = futures::future::join_all(handles).await;

                Ok(Self {
                    ancestors: vec![],
                    descendants,
                })
            },
            None => Err(AppError::not_found("Record", post_id.as_ref())),
        }
    }
}
