use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

mod note;
pub use note::CreateOrUpdateNote;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum CreateOrUpdateType {
    Create,
    Update,
}

impl Display for CreateOrUpdateType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Create => f.write_str("Create"),
            Self::Update => f.write_str("Update"),
        }
    }
}
