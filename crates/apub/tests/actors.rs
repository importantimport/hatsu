use hatsu_apub::{actors::Service, tests::test_asset};
use hatsu_utils::AppError;

#[test]
fn test_parse_actors() -> Result<(), AppError> {
    test_asset::<Service>("assets/gotosocial/actors/kwa_hyp3r.link.json")?;

    Ok(())
}
