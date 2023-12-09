
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
pub async fn setup(config: &mut Config) -> Result<(), Box<dyn std::error::Error>> {
    let target_dir = config_set_target_dir(config);
    let target_web_assets_dir = target_dir.join("web-assets");
    let web_public_dir = config.project_dir.join("web-public");
    // Ensure target/web-assets directory exists
    fs::create_dir_all(target_web_assets_dir.join("wasm"))?;

    // Copy files from web-public to target/web-assets
    if let Ok(entries) = fs::read_dir(web_public_dir) {
        for entry in entries {
            let entry = entry?;
            let dest_path = format!(
                "{}/{}",
                &target_web_assets_dir.to_str().ok_or(ERR_MSG_PATH_TO_STR)?,
                entry.file_name().to_str().ok_or(ERR_MSG_PATH_TO_STR)?
            );
            fs::copy(entry.path(), dest_path)?;
        }
    }

    // Compile wasm-project
    Command::new("wasm-pack")
        .args([
            "build",
            #[cfg(feature = "debug-compilation-wasm-pack")]
            "--dev",
            config.project_dir.to_str().ok_or(ERR_MSG_PATH_TO_STR)?,
            "--target",
            "web",
            "--out-dir",
            target_web_assets_dir
                .join("pkg")
                .to_str()
                .ok_or(ERR_MSG_PATH_TO_STR)?,
        ])
        .status()
        .expect("Failed to compile wasm-project");
    
    // Modify the generated JS glue file
    modify_glue::modify_glue_js(config)?;


    Ok(())
}
