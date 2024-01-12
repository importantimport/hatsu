use std::ops::Deref;

use hatsu_db_schema::received_follow::Model as DbReceivedFollow;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApubReceivedFollow(pub(crate) DbReceivedFollow);

impl AsRef<DbReceivedFollow> for ApubReceivedFollow {
    fn as_ref(&self) -> &DbReceivedFollow {
        &self.0
    }
}

impl Deref for ApubReceivedFollow {
    type Target = DbReceivedFollow;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<DbReceivedFollow> for ApubReceivedFollow {
    fn from(u: DbReceivedFollow) -> Self {
        Self(u)
    }
}
