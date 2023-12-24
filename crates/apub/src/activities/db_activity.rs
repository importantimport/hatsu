use hatsu_db_schema::activity::Model as DbActivity;
use std::ops::Deref;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApubActivity(pub(crate) DbActivity);

impl Deref for ApubActivity {
    type Target = DbActivity;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<DbActivity> for ApubActivity {
    fn from (u: DbActivity) -> Self {
        ApubActivity(u)
    }
}
