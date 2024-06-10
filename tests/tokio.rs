#[cfg(feature = "tokio")]
mod tokio {
    use hydrogen_i18n::builders::TokioI18nBuilder;

    #[tokio::test]
    pub async fn test_tokio_en_translation() {
        let builder = TokioI18nBuilder::new("en");
        builder.add_from_dir("tests/lang").await.unwrap();

        let i18n = builder.build().await.unwrap();

        assert_eq!(
            i18n.translate("en", "test", "test"),
            "this is a simple test"
        );
    }

    #[tokio::test]
    pub async fn test_tokio_pt_translation() {
        let builder = TokioI18nBuilder::new("en");
        builder.add_from_dir("tests/lang").await.unwrap();

        let i18n = builder.build().await.unwrap();

        assert_eq!(
            i18n.translate("pt", "test", "test"),
            "isso é um simples teste"
        );
    }

    #[tokio::test]
    pub async fn test_tokio_default_translation() {
        let builder = TokioI18nBuilder::new("en");
        builder.add_from_dir("tests/lang").await.unwrap();

        let i18n = builder.build().await.unwrap();

        assert_eq!(
            i18n.translate("de", "test", "test"),
            "this is a simple test"
        );
    }

    #[tokio::test]
    pub async fn test_tokio_missing_translation() {
        let builder = TokioI18nBuilder::new("en");
        builder.add_from_dir("tests/lang").await.unwrap();

        let i18n = builder.build().await.unwrap();

        assert_eq!(i18n.translate("en", "test", "missing"), "test.missing");
    }

    #[tokio::test]
    pub async fn test_tokio_link_translation() {
        let builder = TokioI18nBuilder::new("en");
        builder.add_from_dir("tests/lang").await.unwrap();

        let i18n = builder.build().await.unwrap();

        assert_eq!(
            i18n.translate("pt-br", "test", "test"),
            "isso é um simples teste"
        );
    }
}
