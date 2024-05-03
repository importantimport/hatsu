use std::ops::Deref;

use hatsu_db_schema::received_like::Model as DbReceivedLike;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApubReceivedLike(pub(crate) DbReceivedLike);

impl AsRef<DbReceivedLike> for ApubReceivedLike {
    fn as_ref(&self) -> &DbReceivedLike {
        &self.0
    }
}

impl Deref for ApubReceivedLike {
    type Target = DbReceivedLike;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<DbReceivedLike> for ApubReceivedLike {
    fn from(u: DbReceivedLike) -> Self {
        Self(u)
    }
}
