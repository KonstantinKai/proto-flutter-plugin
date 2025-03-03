use std::collections::HashMap;

use extism_pdk::*;
use proto_pdk::*;
use schematic::SchemaBuilder;

use crate::{FlutterDist, FlutterPluginConfig, PubspecYaml};

#[host_fn]
extern "ExtismHost" {
    fn exec_command(input: Json<ExecCommandInput>) -> Json<ExecCommandInput>;
}

static NAME: &str = "Flutter";

#[plugin_fn]
pub fn register_tool(Json(_): Json<RegisterToolInput>) -> FnResult<Json<RegisterToolOutput>> {
    Ok(Json(RegisterToolOutput {
        name: NAME.into(),
        minimum_proto_version: Some(Version::new(0, 47, 0)),
        type_of: PluginType::CommandLine,
        default_install_strategy: InstallStrategy::DownloadPrebuilt,
        config_schema: Some(SchemaBuilder::build_root::<FlutterPluginConfig>()),
        self_upgrade_commands: vec!["upgrade".into(), "downgrade".into()],
        plugin_version: Version::parse(env!("CARGO_PKG_VERSION")).ok(),
        ..RegisterToolOutput::default()
    }))
}

#[plugin_fn]
pub fn load_versions(Json(_): Json<LoadVersionsInput>) -> FnResult<Json<LoadVersionsOutput>> {
    let env = get_host_environment()?;

    let response = fetch_dist(&env)?;
    let mut output = LoadVersionsOutput::default();

    for item in response.releases.iter() {
        // Filter out versions that starts with "v", old versions, dev releases
        if item.version.starts_with("v") || item.version.starts_with("0") || item.channel.eq("dev")
        {
            continue;
        }

        let unresolved_spec = UnresolvedVersionSpec::parse(&item.version)?;

        if response.latest.stable == item.hash {
            output.latest = Some(unresolved_spec.clone());
            output
                .aliases
                .insert("latest".into(), unresolved_spec.clone());
            output
                .aliases
                .insert("stable".into(), unresolved_spec.clone());
        }

        if response.latest.beta == item.hash {
            output
                .aliases
                .insert("beta".into(), unresolved_spec.clone());
        }

        let resolved_spec = unresolved_spec.to_resolved_spec();

        if !output.versions.contains(&resolved_spec) {
            output.versions.push(resolved_spec.clone());
        }
    }

    Ok(Json(output))
}

#[plugin_fn]
pub fn download_prebuilt(
    Json(input): Json<DownloadPrebuiltInput>,
) -> FnResult<Json<DownloadPrebuiltOutput>> {
    let env = get_host_environment()?;

    check_supported_os_and_arch(
        NAME,
        &env,
        permutations! [
            HostOS::Linux => [HostArch::X64],
            HostOS::MacOS => [HostArch::X64, HostArch::Arm64],
            HostOS::Windows => [HostArch::X64],
        ],
    )?;

    let input_version = input.context.version;

    if input_version.is_canary() {
        return Err(plugin_err!(PluginError::Message(format!(
            "{NAME} does not support canary/nightly versions. Plase use `proto install flutter beta` instead"
        ))));
    }

    let os = get_os_as_str(&env);
    let version = input_version.as_version().unwrap();
    let base_url = get_tool_config::<FlutterPluginConfig>()?.base_url;
    let (version_as_str, channel) = (
        version.to_string(),
        if !version.pre.is_empty() || !version.build.is_empty() {
            "beta"
        } else {
            "stable"
        },
    );
    let arch = get_arch_as_str(&env, &input_version, channel);

    // TODO: Not ideal, but this is the only solution at the moment
    let response = fetch_dist(&env)?;
    let checksum = response.releases.iter().find_map(|item| {
        if item.version == version_as_str && item.channel == channel {
            if arch == "arm64_" {
                if item.arch == Some("arm64".into()) {
                    return Some(item.sha256.clone());
                } else {
                    return None;
                }
            }

            Some(item.sha256.clone())
        } else {
            None
        }
    });

    let download_url =
        format!("{base_url}/{channel}/{os}/flutter_{os}_{arch}{version_as_str}-{channel}.zip");

    Ok(Json(DownloadPrebuiltOutput {
        download_url,
        checksum,
        ..DownloadPrebuiltOutput::default()
    }))
}

#[plugin_fn]
pub fn locate_executables(
    Json(_): Json<LocateExecutablesInput>,
) -> FnResult<Json<LocateExecutablesOutput>> {
    let env = get_host_environment()?;

    Ok(Json(LocateExecutablesOutput {
        exes: HashMap::from_iter([
            (
                "flutter".into(),
                ExecutableConfig::new_primary(
                    env.os
                        .for_native("flutter/bin/flutter", "flutter/bin/flutter.bat"),
                ),
            ),
            (
                "dart".into(),
                ExecutableConfig::new(
                    env.os
                        .for_native("flutter/bin/dart", "flutter/bin/dart.bat"),
                ),
            ),
        ]),
        globals_lookup_dirs: vec!["$FLUTTER_ROOT/bin".into()],
        ..LocateExecutablesOutput::default()
    }))
}

#[plugin_fn]
pub fn detect_version_files(_: ()) -> FnResult<Json<DetectVersionOutput>> {
    Ok(Json(DetectVersionOutput {
        // TODO: Add fvm support
        files: vec!["pubspec.yml".into(), "pubspec.yaml".into()],
        ignore: vec![],
    }))
}

#[plugin_fn]
pub fn pre_run(Json(input): Json<RunHook>) -> FnResult<Json<RunHookResult>> {
    let result = RunHookResult::default();
    let args = &input.passthrough_args;

    match args[0].as_str() {
        "channel" if args.len() == 2 => Err(plugin_err!(PluginError::Message(format!(
            "{NAME} does not support channel switching with proto. Plase use `proto install flutter beta` or check it out with git manualy instead. See https://docs.flutter.dev/release/archive#main-channel"
        )))),
        _ => Ok(Json(result))
    }
}

#[plugin_fn]
pub fn parse_version_file(
    Json(input): Json<ParseVersionFileInput>,
) -> FnResult<Json<ParseVersionFileOutput>> {
    let mut version = None;

    if input.file.starts_with("pubspec") {
        let pubspec: PubspecYaml = serde_yml::from_str(&input.content)?;

        if let Some(env) = pubspec.environment {
            if let Some(constraint) = env.flutter {
                version = Some(UnresolvedVersionSpec::parse(constraint)?);
            }
        }
    }

    Ok(Json(ParseVersionFileOutput { version }))
}

fn get_os_as_str(env: &HostEnvironment) -> String {
    match env.os {
        HostOS::MacOS => "macos".into(),
        HostOS::Windows => "windows".into(),
        _ => "linux".into(),
    }
}

fn get_arch_as_str(env: &HostEnvironment, version: &VersionSpec, channel: &str) -> String {
    let empty = String::from("");

    match env.os {
        HostOS::MacOS => {
            if env.arch == HostArch::Arm64 {
                let m1_compat_stable_version = VersionSpec::parse("3.0.0").ok().unwrap();
                let m1_compat_beta_version = VersionSpec::parse("2.12.0-4.1.pre").ok().unwrap();
                let arch = String::from("arm64_");

                match true {
                    _ if channel == "stable" && version.lt(&m1_compat_stable_version) => empty,
                    _ if channel == "beta" && version.lt(&m1_compat_beta_version) => empty,
                    _ => arch,
                }
            } else {
                empty
            }
        }
        _ => empty,
    }
}

fn fetch_dist(env: &HostEnvironment) -> AnyResult<FlutterDist> {
    let suffix = get_os_as_str(env);
    let base_url = get_tool_config::<FlutterPluginConfig>()?.base_url;

    fetch_json::<String, FlutterDist>(format!("{base_url}/releases_{suffix}.json"))
}
