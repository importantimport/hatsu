use activitypub_federation::{
    axum::inbox::{receive_activity, ActivityData},
    config::Data,
    protocol::context::WithContext,
};
use axum::{debug_handler, response::IntoResponse};
use hatsu_apub::{activities::UserInboxActivities, actors::ApubUser};
use hatsu_utils::{AppData, AppError};

use crate::TAG;

/// User inbox
#[utoipa::path(
    post,
    tag = TAG,
    path = "/users/{user}/inbox",
    responses(
        (status = OK),
        (status = NOT_FOUND, body = AppError),
        (status = INTERNAL_SERVER_ERROR, body = AppError)
    ),
    params(
        ("user" = String, Path, description = "The Domain of the User in the database.")
    )
)]
#[debug_handler]
pub async fn handler(data: Data<AppData>, activity_data: ActivityData) -> impl IntoResponse {
    receive_activity::<WithContext<UserInboxActivities>, ApubUser, AppData>(activity_data, &data)
        .await
}
