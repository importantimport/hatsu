
use activitypub_federation::config::Data;
use sea_orm::*;
use url::Url;
// use uuid::Uuid;

use crate::{
  AppData,
  error::AppError,
  entities::{models::{
    prelude::*,
    user_follower::Model as DbUserFollower
  }, user_follower},
};

impl DbUserFollower {
  // 添加关注
  pub async fn new(
    _user_id: Url,
    _follower_id: Url,
    _data: &Data<AppData>
  ) -> Result<(), AppError> {
    Ok(())
  }

  // 查找对应用户名的关注者
  pub async fn find_by_user_id(
    user_id: Url,
    data: &Data<AppData>
  ) -> Result<Vec<DbUserFollower>, AppError> {
    Ok(UserFollower::find()
      .filter(user_follower::Column::UserId.contains(user_id))
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
      .filter(user_follower::Column::FollowerId.contains(follower_id))
      // TODO: 按添加时间排序?
      .order_by_asc(user_follower::Column::UserId)
      .all(&data.conn)
      .await?)
  }
}
