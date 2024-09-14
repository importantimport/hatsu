use activitypub_federation::{
    axum::json::FederationJson,
    config::Data,
    protocol::context::WithContext,
    traits::Object,
};
use axum::{debug_handler, extract::Path, response::Redirect};
use hatsu_apub::objects::{ApubPost, Note};
use hatsu_db_schema::prelude::Post;
use hatsu_utils::{AppData, AppError};
use sea_orm::EntityTrait;

use crate::TAG;

/// Get post
#[utoipa::path(
    get,
    tag = TAG,
    path = "/posts/{post}",
    responses(
        (status = OK, description = "Post", body = Note),
        (status = NOT_FOUND, description = "Post does not exist", body = AppError)
    ),
    params(
        ("post" = String, Path, description = "The Url of the Post in the database.")
    )
)]
#[debug_handler]
pub async fn post(
    Path(post_id): Path<String>,
    data: Data<AppData>,
) -> Result<FederationJson<WithContext<Note>>, AppError> {
    tracing::info!("Reading post {}", post_id);

    let post_url = hatsu_utils::url::generate_post_url(data.domain(), post_id)?;

    match Post::find_by_id(post_url.to_string())
        .one(&data.conn)
        .await?
    {
        Some(db_post) => {
            let apub_post: ApubPost = db_post.into();
            Ok(FederationJson(WithContext::new_default(
                apub_post.into_json(&data).await?,
            )))
        },
        None => Err(AppError::not_found("Post", post_url.as_ref())),
    }
}

#[debug_handler]
pub async fn redirect(Path(post_id): Path<String>) -> Redirect {
    Redirect::permanent(&format!("/posts/{post_id}"))
}
