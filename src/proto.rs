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
        if let Ok(version_spec) = VersionSpec::parse(&item.version) {
            let version_as_option = version_spec.as_version();

            match version_as_option {
                Some(version) => {
                    if version.major == 0
                        || (item.channel.eq("beta")
                            && (version.pre.is_empty() && version.build.is_empty()))
                        || (item.channel.eq("stable") && !version.build.is_empty())
                        || check_version_for_os_and_arch(&env, &version_spec).is_err()
                        || item.channel.eq("dev")
                        || output.versions.contains(&version_spec)
                    {
                        continue;
                    }
                }
                _ => {
                    continue;
                }
            }

            let unresolved_version_spec = version_spec.to_unresolved_spec();

            if response.latest.stable == item.hash {
                output.latest = Some(unresolved_version_spec.clone());
                output
                    .aliases
                    .insert("latest".into(), unresolved_version_spec.clone());
                output
                    .aliases
                    .insert("stable".into(), unresolved_version_spec.clone());
            }

            if response.latest.beta == item.hash {
                output
                    .aliases
                    .insert("beta".into(), unresolved_version_spec.clone());
            }

            output.versions.push(version_spec.clone());
        }
    }

    Ok(Json(output))
}

#[plugin_fn]
pub fn download_prebuilt(
    Json(input): Json<DownloadPrebuiltInput>,
) -> FnResult<Json<DownloadPrebuiltOutput>> {
    let env = get_host_environment()?;
    let version_spec = input.context.version.as_ref();

    check_version_for_os_and_arch(&env, version_spec)?;

    if version_spec.is_canary() {
        return Err(plugin_err!(PluginError::Message(format!(
            "{NAME} does not support canary/nightly versions. Please use `proto install flutter beta` instead"
        ))));
    }

    let base_url = get_tool_config::<FlutterPluginConfig>()?.base_url;
    let os = get_os_as_str(&env);
    let version = version_spec.as_version().unwrap();
    let version_as_string = version.to_string();
    let channel = if !version.pre.is_empty() || !version.build.is_empty() {
        "beta"
    } else {
        "stable"
    };
    // Flutter archives before 1.17.0 (stable) / 1.17.0-dev.3.1 (beta) used a "v" prefix
    // in the filename (e.g. flutter_macos_v1.2.1-stable.zip). Starting from 1.17.0 the
    // prefix was dropped. See https://docs.flutter.dev/release/archive
    let version_v_prefix = if (channel == "stable"
        && version_spec.lt(VersionSpec::parse("1.17.0").unwrap().as_ref()))
        || (channel == "beta"
            && version_spec.lt(VersionSpec::parse("1.17.0-dev.3.1").unwrap().as_ref()))
    {
        "v"
    } else {
        ""
    };
    let arch = if env.arch == HostArch::Arm64 {
        "arm64_"
    } else {
        ""
    };

    let response = fetch_dist(&env)?;
    let checksum = response.releases.iter().find_map(|item| {
        if item.version == format!("{}{}", version_v_prefix, version_as_string)
            && item.channel == channel
        {
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

    let ext = if env.os == HostOS::Linux {
        "tar.xz"
    } else {
        "zip"
    };

    let download_url =
        format!("{base_url}/{channel}/{os}/flutter_{os}_{arch}{version_v_prefix}{version_as_string}-{channel}.{ext}");

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
            "{NAME} does not support channel switching with proto. Please use `proto install flutter beta` or check it out with git manually instead. See https://docs.flutter.dev/release/archive#main-channel"
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

pub fn check_version_for_os_and_arch(
    env: &HostEnvironment,
    version_spec: &VersionSpec,
) -> FnResult<()> {
    let version = version_spec.as_version().unwrap();

    let unresolved_version_spec_option = match env.os {
        HostOS::Linux => match env.arch {
            HostArch::X64 => None::<UnresolvedVersionSpec>,
            _ => UnresolvedVersionSpec::parse("0.0.0").ok(),
        },
        // Flutter added macOS ARM64 (Apple Silicon) support progressively:
        // - Beta/pre-release builds: available from 2.12.0-4.1.pre (first M1 beta)
        // - Stable builds: available from 3.0.0 (first stable with arm64 archives)
        HostOS::MacOS => match env.arch {
            HostArch::Arm64
                if !version.pre.is_empty()
                    && version_spec.lt(VersionSpec::Semantic(SemVer(
                        Version::parse("2.12.0-4.1.pre").ok().unwrap(),
                    ))
                    .as_ref()) =>
            {
                UnresolvedVersionSpec::parse(">=2.12.0-4.1.pre").ok()
            }
            HostArch::Arm64
                if version_spec
                    .lt(VersionSpec::Semantic(SemVer(Version::new(3, 0, 0))).as_ref()) =>
            {
                UnresolvedVersionSpec::parse(">=3.0.0").ok()
            }
            _ => None::<UnresolvedVersionSpec>,
        },
        HostOS::Windows => match env.arch {
            HostArch::X64 => None::<UnresolvedVersionSpec>,
            _ => UnresolvedVersionSpec::parse("0.0.0").ok(),
        },
        _ => UnresolvedVersionSpec::parse("0.0.0").ok(),
    };

    match unresolved_version_spec_option {
        Some(unresolved_version_spec) => match unresolved_version_spec {
            UnresolvedVersionSpec::Req(req) => {
                let arch = env.arch.to_string();

                Err(plugin_err!(PluginError::Message(format!(
                    "Unable to install {NAME}@{version} for the current architecture {arch} and os. Require {req}"
                ))))
            }
            _ => Err(PluginError::UnsupportedOS {
                tool: NAME.to_owned(),
                os: env.os.to_string(),
            }
            .into()),
        },
        _ => Ok(()),
    }
}

fn get_os_as_str(env: &HostEnvironment) -> String {
    match env.os {
        HostOS::MacOS => "macos".into(),
        HostOS::Windows => "windows".into(),
        _ => "linux".into(),
    }
}

fn fetch_dist(env: &HostEnvironment) -> AnyResult<FlutterDist> {
    let suffix = get_os_as_str(env);
    let base_url = get_tool_config::<FlutterPluginConfig>()?.base_url;
    let url = format!("{base_url}/releases_{suffix}.json");
    let cache_key = format!("dist_{suffix}");

    if let Ok(Some(cached)) = var::get::<Vec<u8>>(&cache_key) {
        if let Ok(dist) = json::from_slice::<FlutterDist>(&cached) {
            return Ok(dist);
        }
    }

    let bytes = fetch_bytes(&url)?;
    let _ = var::set(&cache_key, &bytes);
    let dist: FlutterDist = json::from_slice(&bytes)?;

    Ok(dist)
}
