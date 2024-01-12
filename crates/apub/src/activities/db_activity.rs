use std::ops::Deref;

use hatsu_db_schema::activity::Model as DbActivity;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApubActivity(pub(crate) DbActivity);

impl AsRef<DbActivity> for ApubActivity {
    fn as_ref(&self) -> &DbActivity {
        &self.0
    }
}

impl Deref for ApubActivity {
    type Target = DbActivity;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<DbActivity> for ApubActivity {
    fn from(u: DbActivity) -> Self {
        Self(u)
    }
}
