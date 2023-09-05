use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

mod note;
pub use note::CreateOrUpdateNote;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum CreateOrUpdateType {
    Create,
    Update
}

impl Display for CreateOrUpdateType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CreateOrUpdateType::Create => f.write_str("Create"),
            CreateOrUpdateType::Update => f.write_str("Update"),
        }
    }
}
