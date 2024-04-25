use activitypub_federation::{
    axum::json::FederationJson,
    config::Data,
    protocol::context::WithContext,
    traits::Object,
};
use axum::{debug_handler, extract::Path, response::Redirect};
// use axum_extra::routing::TypedPath;
use hatsu_apub::objects::{ApubPost, Note};
use hatsu_db_schema::prelude::Post;
use hatsu_utils::{AppData, AppError};
use sea_orm::EntityTrait;
// use serde::Deserialize;

// #[derive(TypedPath, Deserialize)]
// #[typed_path("/o/*object")]
// pub struct Objects {
//     object: String
// }

// #[derive(TypedPath, Deserialize)]
// #[typed_path("/objects/*object")]
// pub struct ObjectsRedirect {
//     object: String
// }

/// Get post
#[utoipa::path(
    get,
    tag = "apub",
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
    // Objects { object }: Objects,
    Path(post_id): Path<String>,
    data: Data<AppData>,
) -> Result<FederationJson<WithContext<Note>>, AppError> {
    tracing::info!("Reading post {}", post_id);

    let post_url = hatsu_utils::url::generate_post_url(data.domain(), post_id)?;

    match Post::find_by_id(&post_url.to_string())
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
pub async fn redirect(
    // ObjectsRedirect { object }: ObjectsRedirect
    Path(post_id): Path<String>,
) -> Redirect {
    Redirect::permanent(&format!("/posts/{post_id}"))
}
