use std::collections::HashMap;

use extism_pdk::*;
use proto_pdk::*;

static NAME: &str = "buf";

#[plugin_fn]
pub fn register_tool(_input: ()) -> FnResult<Json<RegisterToolOutput>> {
    Ok(Json(RegisterToolOutput {
        name: NAME.into(),
        type_of: PluginType::CommandLine,
        ..RegisterToolOutput::default()
    }))
}

#[plugin_fn]
pub fn load_versions(_input: Json<LoadVersionsInput>) -> FnResult<Json<LoadVersionsOutput>> {
    let tags = load_git_tags("https://github.com/bufbuild/buf")?;
    Ok(Json(LoadVersionsOutput::from(tags)?))
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
            HostOS::Linux => [HostArch::X64, HostArch::Arm64],
            HostOS::MacOS => [HostArch::X64, HostArch::Arm64],
            HostOS::Windows => [HostArch::X64, HostArch::Arm64],
        ],
    )?;

    let version = &input.context.version;

    let os = match env.os {
        HostOS::Linux => "Linux",
        HostOS::MacOS => "Darwin",
        HostOS::Windows => "Windows",
        _ => unreachable!(),
    };

    let arch = match env.os {
        // Linux uses aarch64, not arm64
        HostOS::Linux => match env.arch {
            HostArch::Arm64 => "aarch64",
            HostArch::X64 => "x86_64",
            _ => unreachable!(),
        },
        // macOS and Windows use arm64
        _ => match env.arch {
            HostArch::Arm64 => "arm64",
            HostArch::X64 => "x86_64",
            _ => unreachable!(),
        },
    };

    let ext = if env.os == HostOS::Windows {
        ".exe"
    } else {
        ""
    };

    let filename = format!("buf-{os}-{arch}{ext}");

    Ok(Json(DownloadPrebuiltOutput {
        download_url: format!(
            "https://github.com/bufbuild/buf/releases/download/v{version}/{filename}"
        ),
        download_name: Some(filename),
        checksum_url: Some(format!(
            "https://github.com/bufbuild/buf/releases/download/v{version}/sha256.txt"
        )),
        ..DownloadPrebuiltOutput::default()
    }))
}

#[plugin_fn]
pub fn locate_executables(
    _input: Json<LocateExecutablesInput>,
) -> FnResult<Json<LocateExecutablesOutput>> {
    let env = get_host_environment()?;

    Ok(Json(LocateExecutablesOutput {
        exes: HashMap::from_iter([(
            "buf".into(),
            ExecutableConfig::new_primary(env.os.for_native("buf", "buf.exe")),
        )]),
        ..LocateExecutablesOutput::default()
    }))
}