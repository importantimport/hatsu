use hatsu_apub::{
    activities::{CreateOrUpdateNote, Follow, LikeOrAnnounce, UndoFollow, UndoLikeOrAnnounce},
    tests::test_asset,
};
use hatsu_utils::AppError;

#[test]
fn test_parse_activities() -> Result<(), AppError> {
    test_asset::<CreateOrUpdateNote>("assets/mastodon/activities/create_note.json")?;
    test_asset::<Follow>("assets/mastodon/activities/follow.json")?;
    test_asset::<LikeOrAnnounce>("assets/mastodon/activities/like_page.json")?;
    test_asset::<UndoFollow>("assets/mastodon/activities/undo_follow.json")?;
    test_asset::<UndoLikeOrAnnounce>("assets/mastodon/activities/undo_like_page.json")?;

    Ok(())
}
