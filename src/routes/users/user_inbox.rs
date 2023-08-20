use activitypub_federation::{
    axum::inbox::{receive_activity, ActivityData},
    config::Data,
    protocol::context::WithContext,
};
use axum::{
    debug_handler,
    response::IntoResponse
};

use crate::{
    AppData,
    protocol::activities::activity_lists::PersonInboxActivities,
    entities::user::Model as DbUser,
};

#[debug_handler]
pub async fn handler(
    data: Data<AppData>,
    activity_data: ActivityData
) -> impl IntoResponse {
    receive_activity::<WithContext<PersonInboxActivities>, DbUser, AppData>(
        activity_data,
        &data,
    ).await
}
