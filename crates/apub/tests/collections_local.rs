use hatsu_apub::collections::{
    Collection,
    // TODO: test collection page
    // CollectionPage
};
use hatsu_utils::AppError;
use url::Url;

#[test]
fn new_collection() -> Result<(), AppError> {
    let collection_id = Url::parse("https://hatsu.local/collections/test1")?;
    let collection = Collection::new(&collection_id, 100, 10)?;

    let expected_first_url = Url::parse("https://hatsu.local/collections/test1?page=1")?;
    let expected_last_url = Url::parse("https://hatsu.local/collections/test1?page=10")?;

    assert_eq!(collection.first, expected_first_url);
    assert_eq!(collection.last, expected_last_url);

    Ok(())
}

#[test]
fn new_empty_collection() -> Result<(), AppError> {
    let collection_id = Url::parse("https://hatsu.local/collections/test2")?;
    let collection = Collection::new(&collection_id, 0, 0)?;

    let expected_url = Url::parse("https://hatsu.local/collections/test2?page=1")?;

    assert_eq!(collection.first, expected_url);
    assert_eq!(collection.last, expected_url);

    Ok(())
}
