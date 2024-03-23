use std::fmt::{Display, Formatter, Result};

use activitypub_federation::kinds::activity::{CreateType, UpdateType};
use serde::{Deserialize, Serialize};

mod note;
pub use note::CreateOrUpdateNote;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum CreateOrUpdateType {
    CreateType(CreateType),
    UpdateType(UpdateType),
}

impl Display for CreateOrUpdateType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::CreateType(_) => f.write_str(&CreateType::Create.to_string()),
            Self::UpdateType(_) => f.write_str(&UpdateType::Update.to_string()),
        }
    }
}
