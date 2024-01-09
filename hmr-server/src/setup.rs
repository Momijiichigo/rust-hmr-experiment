use anyhow::Context;
use axum::{
    routing::{get, get_service},
    Router, Server,
};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{collections::HashSet, fs};
use tower_http::services::ServeDir;

use crate::*;

fn config_set_target_dir(config: &mut Config) -> PathBuf {
    config.target_dir.to_owned().unwrap_or_else(|| {
        let target_dir = config.project_dir.join("target").to_path_buf();
        config.target_dir = Some(target_dir.clone());
        target_dir
    })
}
pub async fn setup(config: &mut Config) -> anyhow::Result<()> {
    let target_dir = config_set_target_dir(config);
    let target_web_assets_dir = target_dir.join("web-assets");
    let web_public_dir = config.project_dir.join("web-public");
    // Ensure target/web-assets directory exists
    fs::create_dir_all(target_web_assets_dir.join("wasm"))
        .context("could not create dir: `target/web-assets/wasm`")?;

    // Copy files from web-public to target/web-assets
    if let Ok(entries) = fs::read_dir(web_public_dir) {
        for entry in entries {
            let entry = entry?;
            let dest_path = format!(
                "{}/{}",
                &target_web_assets_dir
                    .to_str()
                    .context(ERR_MSG_PATH_TO_STR)?,
                entry.file_name().to_str().context(ERR_MSG_PATH_TO_STR)?
            );
            fs::copy(entry.path(), dest_path).context("could not copy file")?;
        }
    }
    // erase target/web-assets/pkg directory
    match fs::remove_dir_all(target_web_assets_dir.join("pkg")) {
        Ok(_) => {}
        Err(e) => {
            if e.kind() != std::io::ErrorKind::NotFound {
                Err(e).context("could not remove dir: `target/web-assets/pkg`")?;
            }
        }
    }

    Command::new("cargo")
        .args([
            "build",
            "--manifest-path",
            config
                .project_dir
                .join("Cargo.toml")
                .to_str()
                .context(ERR_MSG_PATH_TO_STR)?,
            "-Z",
            "unstable-options",
            "--out-dir",
            target_web_assets_dir
                .join("pkg")
                .to_str()
                .context(ERR_MSG_PATH_TO_STR)?,
            "--target",
            "wasm32-unknown-unknown",
            "--release",
        ])
        .status()
        .context("Failed to compile cargo project")?;
    let project_name = config
        .project_dir
        .file_name()
        .context("failed to obtain project name")?
        .to_str()
        .context(ERR_MSG_PATH_TO_STR)?
        .replace('-', "_");
    let wasm_file_path = target_web_assets_dir
        .join("pkg")
        .join(project_name.clone())
        .with_extension("wasm");
    let wasm_file_path_str = wasm_file_path.to_str().context(ERR_MSG_PATH_TO_STR)?;

    Command::new("wasm-bindgen")
        .args([
            "--target",
            "web",
            "--out-dir",
            target_web_assets_dir
                .join("pkg")
                .to_str()
                .context(ERR_MSG_PATH_TO_STR)?,
            "--no-demangle",
            "--keep-lld-exports",
            &wasm_file_path_str,
        ])
        .status()
        .context("Failed to compile wasm-bindgen")?;

    let bindgen_wasm_file_path = target_web_assets_dir
        .join("pkg")
        .join(format!("{project_name}_bg").clone())
        .with_extension("wasm");

    let mut module = modify_wasm::module_from_bytes(
        &fs::read(&bindgen_wasm_file_path).context("could not read bindgen_wasm_file_path")?,
    )
    .context("failed to parse bytes as wasm")?;
    modify_wasm::demangle_funcs(&mut module);

    let mut names = HashSet::new();
    module.exports.iter().for_each(|export| {
        names.insert(export.name.to_string());
    });
    module.funcs.iter_mut().for_each(|func| {
        if let Some(name) = &func.name {
            if !names.contains(name) {
                module.exports.add(name.as_str(), func.id());
                names.insert(name.to_string());
            }
        }
    });
    module.globals.iter().for_each(|global| {
        // TODO: change to reliable way to detect stack pointer
        // by parsing the custom section
        if matches!(global, walrus::Global { ty: walrus::ValType::I32, mutable: true, .. }) {
            module.exports.add("__stack_pointer", global.id());
        }
    });
    module.emit_wasm_file(&bindgen_wasm_file_path)?;

    // Old: Compile wasm-project
    // Command::new("wasm-pack")
    //     .args([
    //         "build",
    //         #[cfg(feature = "debug-compilation-wasm-pack")]
    //         "--dev",
    //         config.project_dir.to_str().context(ERR_MSG_PATH_TO_STR)?,
    //         "--target",
    //         "web",
    //         "--out-dir",
    //         target_web_assets_dir
    //             .join("pkg")
    //             .to_str()
    //             .context(ERR_MSG_PATH_TO_STR)?,
    //         // "--no-demangle",
    //         // "--keep-lld-exports",
    //     ])
    //     .status()
    //     .context("Failed to compile wasm-project")?;

    // Modify the generated JS glue file
    modify_glue::modify_glue_js(config)?;

    Ok(())
}
