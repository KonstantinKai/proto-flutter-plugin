use proto_pdk_test_utils::*;

mod flutter_tool {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn loads_versions_from_dist_url() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("flutter-test").await;

        let output = plugin.load_versions(LoadVersionsInput::default()).await;

        assert!(!output.versions.is_empty());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn sets_latest_stable_beta_alias() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("flutter-test").await;

        let output = plugin.load_versions(LoadVersionsInput::default()).await;

        assert!(output.latest.is_some());
        assert!(output.aliases.contains_key("latest"));
        assert_eq!(output.aliases.get("latest"), output.latest.as_ref());

        assert!(output.aliases.contains_key("stable"));
        assert_eq!(output.aliases.get("stable"), output.latest.as_ref());

        assert!(output.aliases.contains_key("beta"));
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn parses_pubspec() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("flutter-test").await;

        assert_eq!(
            plugin
                .parse_version_file(ParseVersionFileInput {
                    content: r#"
name: "My name"
environment:
  flutter: ">=3.29.0"
                    "#
                    .into(),
                    file: "pubspec.yaml".into(),
                    ..Default::default()
                })
                .await,
            ParseVersionFileOutput {
                version: Some(UnresolvedVersionSpec::parse(">=3.29.0").unwrap()),
            }
        );
    }
}
