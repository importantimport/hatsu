use activitypub_federation::{
    axum::inbox::{receive_activity, ActivityData},
    config::Data,
    protocol::context::WithContext,
};
use axum::{debug_handler, response::IntoResponse};
use hatsu_apub::{activities::UserInboxActivities, actors::ApubUser};
use hatsu_utils::AppData;

#[debug_handler]
pub async fn handler(data: Data<AppData>, activity_data: ActivityData) -> impl IntoResponse {
    receive_activity::<WithContext<UserInboxActivities>, ApubUser, AppData>(activity_data, &data)
        .await
}
