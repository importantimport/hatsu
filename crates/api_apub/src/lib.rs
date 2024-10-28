use hatsu_apub::{
    actors::{PublicKeySchema, User, UserAttachment, UserImage},
    collections::{Collection, CollectionOrPage, CollectionPage},
    links::{Emoji, EmojiIcon, Hashtag, Mention, Tag},
    objects::Note,
};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

pub mod activities;
pub mod posts;
pub mod users;

pub const TAG: &str = "apub";

#[derive(OpenApi)]
#[openapi(
    paths(
        posts::notice::notice,
        posts::post::post,
    ),
    components(schemas(
        PublicKeySchema,
        User,
        UserAttachment,
        UserImage,
        Collection,
        CollectionOrPage,
        CollectionPage,
        Emoji,
        EmojiIcon,
        Hashtag,
        Mention,
        Tag,
        Note,
    )),
    tags((name = TAG, description = "ActivityPub API"))
)]
pub struct ApubApi;

#[must_use]
pub fn routes() -> OpenApiRouter {
    OpenApiRouter::with_openapi(ApubApi::openapi())
        .merge(activities::routes())
        .merge(posts::routes())
        .merge(users::routes())
}
