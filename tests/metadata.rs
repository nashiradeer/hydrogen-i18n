use hydrogen_i18n::metadata::MetadataBuilder;

#[test]
pub fn test_metadata() {
    let metadata = MetadataBuilder::load_dir("tests/data");

    assert_eq!(metadata.len(), 29);
    assert_eq!(metadata.len(), metadata.capacity());
}

#[tokio::test]
pub async fn test_tokio_metadata() {
    let metadata = MetadataBuilder::tokio_load_dir("tests/data").await;

    assert_eq!(metadata.len(), 29);
    assert_eq!(metadata.len(), metadata.capacity());
}
