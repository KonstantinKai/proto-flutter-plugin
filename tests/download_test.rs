use proto_pdk_test_utils::*;

mod flutter_tool {
    use super::*;

    generate_download_install_tests!("flutter-test", "3.29.0");

    #[tokio::test(flavor = "multi_thread")]
    async fn supports_macos_arm64() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("flutter-test", |config| {
                config.host(HostOS::MacOS, HostArch::Arm64);
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
                    "https://storage.googleapis.com/flutter_infra_release/releases/stable/macos/flutter_macos_arm64_3.29.0-stable.zip"
                        .into(),
                checksum: Some("8c3196363c7e79ead5bd2bd657cad6915afdf5b315ca51bfa7e569f490ec3de4".into()),
                ..Default::default()
            }
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
    async fn supports_versions_with_v_prefix_stable() {
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
                        version: VersionSpec::parse("1.2.1").unwrap(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .await,
            DownloadPrebuiltOutput {
                download_url:
                    "https://storage.googleapis.com/flutter_infra_release/releases/stable/macos/flutter_macos_v1.2.1-stable.zip"
                        .into(),
                checksum: Some("74ac8397ea29720f116980ea00cf60c34430be1f64489b407f7cf95553babbef".into()),
                ..Default::default()
            },
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn supports_versions_with_v_prefix_beta() {
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
                        version: VersionSpec::parse("1.12.13+hotfix.6").unwrap(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .await,
            DownloadPrebuiltOutput {
                download_url:
                    "https://storage.googleapis.com/flutter_infra_release/releases/beta/macos/flutter_macos_v1.12.13+hotfix.6-beta.zip"
                        .into(),
                checksum: Some("05c7064de1f793ed1660422bc8f3fc8cdcaed38618bddbee785413b92d80c364".into()),
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
