use activitypub_federation::{
    axum::json::FederationJson,
    config::Data, protocol::context::WithContext,
    // fetch::object_id::ObjectId,
};
use axum::{
    debug_handler,
    extract::{
        Path,
        // Query
    },
    response::{IntoResponse, Redirect},
};
// use sea_orm::*;
// use serde::Deserialize;
use url::Url;

use crate::{
    AppData,
    AppError,
    // entities::{
    //     prelude::*,
    //     activity,
    //     user::Model as DbUser,
    // },
    protocol::collections::outbox::Outbox,
};

// #[derive(Deserialize)]
// pub struct Pagination {
//     page: Option<u64>,
// }

// impl Default for Pagination {
//     fn default() -> Self {
//         Self { page: None }
//     }
// }

#[debug_handler]
pub async fn handler(
    Path(name): Path<String>,
    // pagination: Option<Query<Pagination>>,
    data: Data<AppData>,
) -> Result<FederationJson<WithContext<Outbox>>, AppError> {
    // let Query(pagination) = pagination.unwrap_or_default();

    // let user_id: ObjectId<DbUser> = Url::parse(&format!("https://{}/u/{}", data.domain(), &name))?.into();
    // let user = user_id.dereference_local(&data).await?;

    // let activity_pages = user.find_related(Activity)
    //     // TODO: order by last_refreshed_at
    //     .order_by_asc(activity::Column::Id)
    //     // 20 per page
    //     .paginate(&data.conn, 20);

    // let total = post_pages.num_items_and_pages().await?;

    Ok(FederationJson(WithContext::new_default(
        Outbox::new(
            Url::parse(&format!("https://{}/u/{}/outbox", data.domain(), name))?,
            // total.number_of_items,
            // total.number_of_pages,
            0,
            0,
        )?
    )))
}

#[debug_handler]
pub async fn redirect(Path(name): Path<String>) -> impl IntoResponse {
    Redirect::permanent(&format!("/u/{}/outbox", name)).into_response()
}
