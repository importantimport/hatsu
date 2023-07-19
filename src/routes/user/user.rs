// use activitypub_federation::{
//   axum::{
//       json::FederationJson,
//   },
//   config::Data,
//   protocol::context::WithContext,
// };
// use axum::{
//   extract::Path,
//   // response
// };
// use axum_macros::debug_handler;

// use crate::database::Database;
// use crate::error::Error;

// #[debug_handler]
// pub async fn user(
//     Path(name): Path<String>,
//     data: Data<Database>,
// ) -> Result<FederationJson<WithContext<Person>>, Error> {
//     let db_user = data.read_user(&name)?;
//     let json_user = db_user.into_json(&data).await?;
//     Ok(FederationJson(WithContext::new_default(json_user)))
// }

pub async fn user() -> &'static str {
  "Hello, World!"
}