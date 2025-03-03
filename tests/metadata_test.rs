use proto_pdk_test_utils::*;

mod flutter_tool {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn registers_metadata() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("flutter-test").await;

        let metadata = plugin.register_tool(RegisterToolInput::default()).await;

        assert_eq!(metadata.name, "Flutter");
        assert_eq!(metadata.minimum_proto_version, Some(Version::new(0, 47, 0)));
        assert_eq!(metadata.self_upgrade_commands, vec!["upgrade", "downgrade"]);
        assert_eq!(
            metadata.default_install_strategy,
            InstallStrategy::DownloadPrebuilt
        );
        assert_eq!(
            metadata.plugin_version.unwrap().to_string(),
            env!("CARGO_PKG_VERSION")
        );
    }
}
