use std::ops::Deref;

use hatsu_db_schema::received_announce::Model as DbReceivedAnnounce;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApubReceivedAnnounce(pub(crate) DbReceivedAnnounce);

impl AsRef<DbReceivedAnnounce> for ApubReceivedAnnounce {
    fn as_ref(&self) -> &DbReceivedAnnounce {
        &self.0
    }
}

impl Deref for ApubReceivedAnnounce {
    type Target = DbReceivedAnnounce;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<DbReceivedAnnounce> for ApubReceivedAnnounce {
    fn from(u: DbReceivedAnnounce) -> Self {
        Self(u)
    }
}
