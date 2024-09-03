use std::collections::HashMap;

use hydrogen_i18n::language::Language;

#[test]
pub fn test_language() {
    let en_language = Language::load_file("tests/data/english/en-US.toml").unwrap();
    let en_metadata = en_language.metadata();

    let args = HashMap::from([(String::from("name"), String::from("Nashira Deer"))]);

    assert_eq!(en_metadata.name(), Some("English (United States)"));
    assert_eq!(en_metadata.code(), Some("en-US"));

    assert_eq!(en_language.translate("hello", "world"), "Hello, world!");
    assert_eq!(en_language.translate("hello", "invalid"), "hello.invalid");
    assert_eq!(
        en_language.translate_with("hello", "user", args.clone().into_iter()),
        "Hello, Nashira Deer!"
    );

    let pt_language = Language::load_file("tests/data/portuguese/pt-BR.toml").unwrap();
    let pt_metadata = pt_language.metadata();

    assert_eq!(pt_metadata.name(), Some("Português (Brasil)"));
    assert_eq!(pt_metadata.code(), Some("pt-BR"));

    assert_eq!(pt_language.translate("hello", "world"), "Olá, mundo!");
    assert_eq!(pt_language.translate("hello", "invalid"), "hello.invalid");
    assert_eq!(
        pt_language.translate_with("hello", "user", args.clone().into_iter()),
        "Olá, Nashira Deer!"
    );
}

#[tokio::test]
pub async fn test_tokio_language() {
    let en_language = Language::tokio_load_file("tests/data/english/en-US.toml")
        .await
        .unwrap();
    let en_metadata = en_language.metadata();

    let args = HashMap::from([(String::from("name"), String::from("Nashira Deer"))]);

    assert_eq!(en_metadata.name(), Some("English (United States)"));
    assert_eq!(en_metadata.code(), Some("en-US"));

    assert_eq!(en_language.translate("hello", "world"), "Hello, world!");
    assert_eq!(en_language.translate("hello", "invalid"), "hello.invalid");
    assert_eq!(
        en_language.translate_with("hello", "user", args.clone().into_iter()),
        "Hello, Nashira Deer!"
    );

    let pt_language = Language::tokio_load_file("tests/data/portuguese/pt-BR.toml")
        .await
        .unwrap();
    let pt_metadata = pt_language.metadata();

    assert_eq!(pt_metadata.name(), Some("Português (Brasil)"));
    assert_eq!(pt_metadata.code(), Some("pt-BR"));

    assert_eq!(pt_language.translate("hello", "world"), "Olá, mundo!");
    assert_eq!(pt_language.translate("hello", "invalid"), "hello.invalid");
    assert_eq!(
        pt_language.translate_with("hello", "user", args.clone().into_iter()),
        "Olá, Nashira Deer!"
    );
}
