use hatsu_apub::{objects::Note, tests::test_asset};
use hatsu_utils::AppError;

#[test]
fn test_parse_notes() -> Result<(), AppError> {
    test_asset::<Note>("assets/gotosocial/objects/note.json")?;

    Ok(())
}
