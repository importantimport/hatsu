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
  entities::user::Model as DbUser,
  objects::user::PersonAcceptedActivities
};

#[debug_handler]
pub async fn user_inbox(
  data: Data<AppData>,
  activity_data: ActivityData
) -> impl IntoResponse {
  receive_activity::<WithContext<PersonAcceptedActivities>, DbUser, AppData>(
    activity_data,
    &data,
  ).await
}