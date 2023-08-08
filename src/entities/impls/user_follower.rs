
use activitypub_federation::config::Data;
use sea_orm::*;
use url::Url;
use uuid::Uuid;

use crate::{
    AppData,
    AppError,
    entities::{
        prelude::*,
        user_follower::{self, Model as DbUserFollower}
    },
};

impl DbUserFollower {
    // 添加关注
    pub async fn insert(
        user_id: Url,
        follower_id: Url,
        data: &Data<AppData>
    ) -> Result<(), AppError> {
        match UserFollower::find()
        .filter(
            Condition::all()
                .add(user_follower::Column::UserId.eq(user_id.to_string()))
                .add(user_follower::Column::FollowerId.eq(follower_id.to_string()))
        )
        .one(&data.conn)
        .await? {
            // TODO: 报错
            Some(_user_follower) => Ok(()),
            None => {
                let user_follower = Self {
                    id: Uuid::now_v7().to_string(),
                    user_id: user_id.to_string(),
                    follower_id: follower_id.to_string(),
                }.into_active_model();

                user_follower.insert(&data.conn).await?;

                Ok(())
            }
        }
    }

    // 查找对应用户名的关注者
    pub async fn find_by_user_id(
        user_id: Url,
        data: &Data<AppData>
    ) -> Result<Vec<DbUserFollower>, AppError> {
        Ok(UserFollower::find()
            .filter(user_follower::Column::UserId.eq(user_id.to_string()))
            // TODO: 按添加时间排序?
            .order_by_asc(user_follower::Column::FollowerId)
            .all(&data.conn)
            .await?)
    }

    // 查找对应用户名关注的账号
    pub async fn find_by_follower_id(
        follower_id: Url,
        data: &Data<AppData>
    ) -> Result<Vec<DbUserFollower>, AppError> {
        Ok(UserFollower::find()
            .filter(user_follower::Column::FollowerId.eq(follower_id.to_string()))
            // TODO: 按添加时间排序?
            .order_by_asc(user_follower::Column::UserId)
            .all(&data.conn)
            .await?)
    }
}
