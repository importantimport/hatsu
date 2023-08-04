use activitypub_federation::kinds::activity::{CreateType, UpdateType};
use serde::{Deserialize, Serialize};

mod note;
pub use note::CreateOrUpdateNote;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum CreateOrUpdateType {
  Create(CreateType),
  Update(UpdateType)
}
