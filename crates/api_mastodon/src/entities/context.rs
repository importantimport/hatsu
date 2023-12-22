use hatsu_utils::AppError;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::entities::Status;

/// https://docs.joinmastodon.org/entities/Context/
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Context {
    /// should always be empty vec
    pub ancestors: Vec<Status>,
    pub descendants: Vec<Status>,
}


impl Context {
    // TODO: String => ObjectId<DbPost>
    pub fn find_by_id(_post_id: String) -> Result<Self, AppError> {
        Ok(Self {
            ancestors: vec![],
            // TODO
            descendants: vec![]
        })
        // https://www.sea-ql.org/SeaORM/docs/relation/chained-relations/
        // let post = find_by_id(post_id)
        // let replies = post.find_linked(post::SelfReferencingLink)
        // let statuses = replies.map(|reply| Status::from(reply))
    }
}
