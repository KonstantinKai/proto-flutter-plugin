use proto_pdk_test_utils::*;

mod flutter_tool {
    use super::*;

    generate_download_install_tests!("flutter-test", "3.29.0");

    #[tokio::test(flavor = "multi_thread")]
    async fn supports_macos_arm64() {
        let sandbox = create_empty_proto_sandbox();
        let plugin_arm = sandbox
            .create_plugin_with_config("flutter-test", |config| {
                config.host(HostOS::MacOS, HostArch::Arm64);
            })
            .await;

        assert_eq!(
            plugin_arm
                .download_prebuilt(DownloadPrebuiltInput {
                    context: ToolContext {
                        version: VersionSpec::parse("3.29.0").unwrap(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .await,
            DownloadPrebuiltOutput {
                download_url:
                    "https://storage.googleapis.com/flutter_infra_release/releases/stable/macos/flutter_macos_arm64_3.29.0-stable.zip"
                        .into(),
                checksum: Some("8c3196363c7e79ead5bd2bd657cad6915afdf5b315ca51bfa7e569f490ec3de4".into()),
                ..Default::default()
            }
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn supports_macos_arm64_without_m1_compat() {
        let sandbox = create_empty_proto_sandbox();
        let plugin_arm = sandbox
            .create_plugin_with_config("flutter-test", |config| {
                config.host(HostOS::MacOS, HostArch::Arm64);
            })
            .await;

        assert_eq!(
            plugin_arm
                .download_prebuilt(DownloadPrebuiltInput {
                    context: ToolContext {
                        // unavailable version
                        version: VersionSpec::parse("2.29.0").unwrap(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .await,
            DownloadPrebuiltOutput {
                download_url:
                    "https://storage.googleapis.com/flutter_infra_release/releases/stable/macos/flutter_macos_2.29.0-stable.zip"
                        .into(),
                ..Default::default()
            },
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn supports_macos_x64() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("flutter-test", |config| {
                config.host(HostOS::MacOS, HostArch::X64);
            })
            .await;

        assert_eq!(
            plugin
                .download_prebuilt(DownloadPrebuiltInput {
                    context: ToolContext {
                        version: VersionSpec::parse("3.29.0").unwrap(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .await,
            DownloadPrebuiltOutput {
                download_url:
                    "https://storage.googleapis.com/flutter_infra_release/releases/stable/macos/flutter_macos_3.29.0-stable.zip"
                        .into(),
                checksum: Some("d3b2d01b7f6713f3a8c3c51ea4a4fb77a2775cfc69708f608bd2ff688493242a".into()),
                ..Default::default()
            },
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn supports_linux_x64() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("flutter-test", |config| {
                config.host(HostOS::Linux, HostArch::X64);
            })
            .await;

        assert_eq!(
            plugin
                .download_prebuilt(DownloadPrebuiltInput {
                    context: ToolContext {
                        version: VersionSpec::parse("3.29.0").unwrap(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .await,
            DownloadPrebuiltOutput {
                download_url:
                    "https://storage.googleapis.com/flutter_infra_release/releases/stable/linux/flutter_linux_3.29.0-stable.zip"
                        .into(),
                checksum: Some("1f98f3de2931e1d097970e56df691b035f6840aa05be632c4fa2a2298c7cfdd8".into()),
                ..Default::default()
            },
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn supports_windows_x64() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("flutter-test", |config| {
                config.host(HostOS::Windows, HostArch::X64);
            })
            .await;

        assert_eq!(
            plugin
                .download_prebuilt(DownloadPrebuiltInput {
                    context: ToolContext {
                        version: VersionSpec::parse("3.29.0").unwrap(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .await,
            DownloadPrebuiltOutput {
                download_url:
                    "https://storage.googleapis.com/flutter_infra_release/releases/stable/windows/flutter_windows_3.29.0-stable.zip"
                        .into(),
                checksum: Some("0b0080912f856b66843a2061bc73e73ab1ea20b68f068100956da69783b4ca70".into()),
                ..Default::default()
            },
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn locates_unix_bin() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("flutter-test", |config| {
                config.host(HostOS::Linux, HostArch::Arm64);
            })
            .await;

        assert_eq!(
            plugin
                .locate_executables(LocateExecutablesInput {
                    context: ToolContext {
                        version: VersionSpec::parse("3.29.0").unwrap(),
                        ..Default::default()
                    },
                })
                .await
                .exes
                .get("flutter")
                .unwrap()
                .exe_path,
            Some("flutter/bin/flutter".into())
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn locates_windows_bin() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("flutter-test", |config| {
                config.host(HostOS::Windows, HostArch::X64);
            })
            .await;

        assert_eq!(
            plugin
                .locate_executables(LocateExecutablesInput {
                    context: ToolContext {
                        version: VersionSpec::parse("3.29.0").unwrap(),
                        ..Default::default()
                    },
                })
                .await
                .exes
                .get("flutter")
                .unwrap()
                .exe_path,
            Some("flutter/bin/flutter.bat".into())
        );
    }
}
