use hatsu_db_schema::received_follow::Model as DbReceivedFollow;
use std::ops::Deref;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApubReceivedFollow(pub(crate) DbReceivedFollow);

impl Deref for ApubReceivedFollow {
    type Target = DbReceivedFollow;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<DbReceivedFollow> for ApubReceivedFollow {
    fn from (u: DbReceivedFollow) -> Self {
        ApubReceivedFollow(u)
    }
}
