use hydrogen_i18n::builders::I18nBuilder;

#[test]
pub fn test_en_translation() {
    let i18n = I18nBuilder::new("en")
        .add_from_dir("tests/lang")
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(
        i18n.translate("en", "test", "test"),
        "this is a simple test"
    );
}

#[test]
pub fn test_pt_translation() {
    let i18n = I18nBuilder::new("en")
        .add_from_dir("tests/lang")
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(
        i18n.translate("pt", "test", "test"),
        "isso é um simples teste"
    );
}

#[test]
pub fn test_default_translation() {
    let i18n = I18nBuilder::new("en")
        .add_from_dir("tests/lang")
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(
        i18n.translate("de", "test", "test"),
        "this is a simple test"
    );
}

#[test]
pub fn test_missing_translation() {
    let i18n = I18nBuilder::new("en")
        .add_from_dir("tests/lang")
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(i18n.translate("en", "test", "missing"), "test.missing");
}

#[test]
pub fn test_link_translation() {
    let i18n = I18nBuilder::new("en")
        .add_from_dir("tests/lang")
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(
        i18n.translate("pt-br", "test", "test"),
        "isso é um simples teste"
    );
}
